"use strict";

/* sse states */
const sseStates = ['connecting', 'open', 'closed'];
const sseMessages = ['Connecting...', 'Connected', 'Disconnected'];
const sseLogFn = [console.warn, console.log, console.error];
const sseStateEl = document.getElementById('sse-state');
const sseStateInnerEl = document.getElementById('sse-state-inner');

function onSseStateChange(state) {
  sseLogFn[state](`[SSE] State changed to ${ sseStates[state] }`);
  for (let i = 0; i < sseStates.length; i++) {
    sseStateEl.classList.toggle(sseStates[i], state === i)
  }
  sseStateInnerEl.textContent = sseMessages[state];
}

const evtSource = new EventSource('/chat/events');
evtSource.addEventListener('open', () => {
  onSseStateChange(evtSource.readyState);
});
evtSource.addEventListener('close', () => {
  onSseStateChange(evtSource.readyState);
});
evtSource.addEventListener('error', error => {
  console.error(error);
  onSseStateChange(evtSource.readyState);
});
evtSource.addEventListener('message', event => {
  console.log("msg event:", event);
  const parsedData = JSON.parse(event.data);
  console.log("msg data:", parsedData);
  if ((jsData.conversationId | 0) == parsedData.conversation_id) {
    addMessage(parsedData.message);
  }
  updateLastMessage(parsedData.conversation_id, parsedData.message.content);
});
onSseStateChange(evtSource.readyState);

/* Dont show sse state for a while */
setTimeout(() => {
  sseStateEl.classList.remove('delay');
}, 500);

/* Render message */
const dummyEl = document.getElementById('message-dummy').firstElementChild;
function renderMessage(message) {
  const dummy = dummyEl.cloneNode(true);
  dummy.classList.add(message.relative_message_direction);
  dummy.id += message.message_id;
  dummy.getElementsByClassName('message-content-content')[0].textContent = message.content;
  dummy.getElementsByClassName('message-content-username')[0].textContent = (
    message.relative_message_direction == "outgoing" ? jsData.userAUsername : jsData.userBUsername
  );
  dummy.getElementsByClassName('chat-message-pfp')[0].src = (
    message.relative_message_direction == "outgoing" ? jsData.userAProfileImage : jsData.userBProfileImage
  ) || (`/dyn/profile_image.svg?usr=${encodeURIComponent(
    message.relative_message_direction == "outgoing" ? jsData.userAUsername : jsData.userBUsername
  )}&id=${encodeURIComponent(
    message.relative_message_direction == "outgoing" ? jsData.userAId : jsData.userBId
  )}`);
  return dummy;
}

/* Add message */
const msgViewInnerEl = document.getElementsByClassName("message-view-inner")[0];
function addMessage(message) {
  //Remove no chat messages message
  msgViewInnerEl.querySelector(".something-gone")?.remove();
  //Only remove last-of-block if same direction
  if (msgViewInnerEl.lastElementChild) {
    if (!(
      (msgViewInnerEl.lastElementChild.classList.contains("incoming") && (message.relative_message_direction == "outgoing")) ||
      (msgViewInnerEl.lastElementChild.classList.contains("outgoing") && (message.relative_message_direction == "incoming"))
    )) {
      msgViewInnerEl.lastElementChild.classList.remove("last-of-block");
    }
  }
  const element = renderMessage(message);
  element.classList.add("last-of-block");
  msgViewInnerEl.appendChild(element);
  return element;
}

/* upd last msg*/
function updateLastMessage(conv_id, messageContent) {
  const query = `.layout-left .conversation-item.conversation-id-${(conv_id | 0).toString()} .conv-last-msg`;
  console.log("last msg query:", query);
  document.querySelector(query).textContent = messageContent;
}

/* MsgBox and message send */
const msgBoxEl = document.getElementsByClassName('message-box')[0];
const sendButtonEl = document.getElementsByClassName('message-box-submit')[0];

sendButtonEl.addEventListener('click', async event => {
  event.preventDefault();
  const formData = new FormData(msgBoxEl);
  const data = {
    content: formData.get('content'),
    conversation_id: parseInt(formData.get('conversation_id')),
  };
  if (!data.content) return;
  //clear out
  msgBoxEl.querySelector(".message-box-input input").value = "";
  const element = addMessage({
    content: data.content,
    relative_message_direction: "outgoing"
  });
  element.classList.add("pending");
  const res = await fetch('/chat/send_message', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });
  if (res && !res.ok) {
    element.remove();
    msgViewInnerEl.lastElementChild.classList.add("last-of-block");
    return;
  }
  element.classList.remove("pending");
  updateLastMessage(jsData.conversationId, data.content);
});
