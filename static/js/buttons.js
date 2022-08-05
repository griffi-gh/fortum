Array.from(document.getElementsByClassName('styled-input')).forEach(field => {
  field.addEventListener('keydown', function( event ) {
    const caps = event.getModifierState && event.getModifierState('CapsLock');
    field.classList.toggle('caps', caps);
  });
})
