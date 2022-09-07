"use strict";

const modal = document.getElementById('share-modal');
const whatEl = document.getElementById('share-what');
const closeEl = document.getElementById('share-modal-header-close');

window.shareModal = async (url, what = '') => {
  const encodedUrl = encodeURIComponent(url);
  modal.classList.remove('display-none');
  modal.ariaHidden = 'false';
  whatEl.textContent = what;
  document.body.parentElement.classList.add('hide-scrollbar');
  await wait();
  modal.classList.add('show');
};

window.closeShareModal = async () => {
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
    await closeShareModal();
  }
});
closeEl.addEventListener('click', async () => {
  await closeShareModal();
});

body.classList.toggle('js-navigator-share', !!navigator.share)
