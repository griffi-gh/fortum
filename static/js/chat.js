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
  console.log(event);
});
onSseStateChange(evtSource.readyState);

/* Dont show sse state for a while */
setTimeout(() => {
  sseStateEl.classList.remove('delay');
}, 500);

/* Render message */
const dummy = document.getElementById('message-dummy').childNodes[0];
function renderMessage(message) {
  const dummy = dummy.cloneNode();
  dummy.classList.add(message.relative_message_direction || message.computed_direction);
  dummy.id += message.message_id;
}

/* MsgBox and message send */
const msgBoxEl = document.getElementsByClassName('message-box')[0];
const sendButtonEl = document.getElementsByClassName('message-box-submit')[0];

sendButtonEl.addEventListener('click', async event => {
  event.preventDefault();
  const formData = new FormData(msgBoxEl);
  await fetch('/chat/send_message', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      conversation_id: parseInt(formData.get('conversation_id')),
      content: formData.get('content'),
    })
  })
});
