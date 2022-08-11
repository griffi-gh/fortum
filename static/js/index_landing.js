const landing = document.getElementById('landing');
const wrapper = document.getElementById('landing-content-wrapper');
const headerRight = document.getElementById('header-right');
const lh = landing.offsetHeight;

const onScroll = () => {
  const scroll = document.body.parentNode.scrollTop;
  landing.style.backgroundPosition = `center calc(50% + ${scroll / 2}px)`;
  wrapper.style.transform = `translateY(${scroll / 1.5}px)`;
  wrapper.style.opacity = Math.max(Math.min(1.5 - (scroll / lh), 1), 0);
  headerRight.classList.toggle('hhide', scroll < 100);
}

{
  let requested = false;
  document.addEventListener('scroll', event => {
    if (requested) return;
    requested = true;
    requestAnimationFrame(() => {
      onScroll();
      requested = false;
    });
  });
}
onScroll();
