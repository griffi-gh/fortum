"use strict";

const form = document.getElementById('reg-form');
if (form) {
  const passwd = document.getElementById('password-input-inner-main');
  const repeat = document.getElementById('password-input-inner-repeat');
  const event_callback = () => {
    const match = passwd.value === repeat.value;
    repeat.setCustomValidity(match ? "": "Passwords don't match");
  };
  form.addEventListener('change', event_callback);
  passwd.addEventListener('input', event_callback);
  repeat.addEventListener('input', event_callback);
}
