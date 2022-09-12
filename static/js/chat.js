"use strict";

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
  console.log(event.data);
});
onSseStateChange(evtSource.readyState);
