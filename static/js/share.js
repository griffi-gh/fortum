"use strict";

const modal = document.getElementById('share-modal');
const whatEl = document.getElementById('share-what');
const closeEl = document.getElementById('share-modal-header-close');
const headerTitleEl = document.getElementById('share-modal-header-title');

const focusStack = [];

window.shareModal = async (url, what = '') => {
  const encodedUrl = encodeURIComponent(url);
  modal.classList.remove('display-none');
  modal.ariaHidden = 'false';
  whatEl.textContent = what;
  document.body.parentElement.classList.add('hide-scrollbar');
  focusStack.push(document.querySelector(':focus'));
  await wait();
  headerTitleEl.focus();
  modal.classList.add('show');
};

window.closeShareModal = async () => {
  focusStack.pop().focus();
  modal.classList.remove('show');
  modal.ariaHidden = 'true';
  document.body.parentElement.classList.remove('hide-scrollbar');
  await wait(1000);
  if (modal.ariaHidden.toString() == 'true') {
    modal.classList.add('display-none');
  }
}

modal.addEventListener('click', async (event) => {
  if (event.target == event.currentTarget) {
    event.preventDefault();
    await closeShareModal();
  }
});
closeEl.addEventListener('click', async (event) => {
  event.preventDefault();
  await closeShareModal();
});

body.classList.toggle('js-navigator-share', !!navigator.share)
