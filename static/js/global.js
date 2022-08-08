Array.from(document.getElementsByClassName("form-with-loading")).forEach(form => {
  form.addEventListener('submit', () => {
    const subm = form.querySelector('button[type="submit"], input[type="submit"]');
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
const hasScrollbar = e => e.scrollHeight > e.clientHeight;
document.body.classList.toggle('no-scrollbar', !hasScrollbar(document.body.parentNode));
