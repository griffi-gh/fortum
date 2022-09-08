// update approx header size accurately
addEventListener('load', () => {
  addEventListener('resize', car(() => {
    const clientRect = document.getElementById('header').getBoundingClientRect();
    document.body.parentElement.style.setProperty('--estim-header-height', `${ clientRect.height }px`);
  }));
});
