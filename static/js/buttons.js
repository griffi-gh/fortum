//Caps lock indicator
{
  let prevCapsState = false;
  const styledFields = document.getElementsByClassName('styled-input');
  document.body.addEventListener('keydown', event => {
    const caps = !!(event.getModifierState && event.getModifierState('CapsLock'));
    if (prevCapsState !== caps) {
      prevCapsState = caps;
      Array.from(styledFields).forEach(field => {
        field.classList.toggle('caps', caps);
      });
    }
  });
}
