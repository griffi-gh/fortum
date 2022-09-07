"use strict";

const landing = document.getElementById('landing');
const background = document.getElementById('landing-bg');
const content = document.getElementById('landing-content');
const headerRight = document.getElementById('header-right');
let lh = landing.offsetHeight;

if (document.body.parentNode.scrollTop < 100) {
  headerRight.classList.add('hhide');
}

const onScroll = () => {
  const scroll = document.body.parentNode.scrollTop;
  if (scroll <= (lh * 2)) {
    const opa = Math.max(Math.min(1.5 - (scroll / lh), 1), 0);
    background.style.transform = `translateY(${scroll / 2}px) scale(${1 + ((1 - opa) / 4)}) translateZ(0)`;
    background.style.filter = `blur(${(1 - opa) * 5}px)`;
    content.style.transform = `translateY(${scroll / 1.5}px) translateZ(0)`;
    content.style.opacity = opa;
    landing.style.transform = 'translateZ(0)';
    landing.style.filter = `brightness(${50 + (opa * 50)}%)`;
    headerRight.classList.toggle('hhide', scroll < 100);
  } else {
    headerRight.classList.remove('hhide');
  }
}

//Throttle callbacks
let requested = false;
document.addEventListener('scroll', () => {
  if (requested) return;
  requested = true;
  requestAnimationFrame(() => {
    onScroll();
    requested = false;
  });
});
window.addEventListener('resize', () => { lh = landing.offsetHeight; });
document.addEventListener('load', () => { lh = landing.offsetHeight; });
