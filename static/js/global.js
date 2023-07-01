"use strict";

// .form-with-loading
Array.from(document.getElementsByClassName("form-with-loading")).forEach(form => {
  form.addEventListener('submit', event => {
    const subm = event.submitter || form.querySelector('button[type="submit"], input[type="submit"]');
    if (subm.getElementsByClassName('styled-button-icons').length > 0) {
      subm.getElementsByClassName('styled-button-icon-selected')[0].classList.remove('styled-button-icon-selected');
      subm.getElementsByClassName('subm-icon-loading')[0].classList.add('styled-button-icon-selected');
    } else {
      console.warn('Button group not found');
    }
    form.disabled = true;
    subm.disabled = true;
  });
});

// body.no-scrollbar
const hasScrollbar = e => e.scrollHeight > e.clientHeight;
const recalcHasScrollbar = () => {
  document.body.parentElement.classList.toggle('no-scrollbar', !hasScrollbar(document.body.parentNode));
};
recalcHasScrollbar();
document.addEventListener('load', recalcHasScrollbar);

// utils
window.wait = (time = 0) => (new Promise((resolve, _reject) => {
  if (time > 0) {
    setTimeout(() => resolve(), time);
  } else {
    requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
  }
}));
window.car = fn => { fn(); return fn };

//get js data
{
  const jsData = {};
  Array.from(document.getElementsByClassName("js-data")).forEach(dataElement => {
    Object.assign(jsData, dataElement?.dataset ?? {});
  });
  console.log("jsData", jsData);
  window.jsData = jsData;
}
