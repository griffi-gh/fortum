const icon0 = document.getElementById("login-icon-cont");
const icon1 = document.getElementById("login-icon-loading");
const form = document.getElementById("login-form");
const subm = document.getElementById("login-submit-btn");

form.addEventListener('submit', () => {
  subm.disabled = true;
  icon0.classList.remove('styled-button-icon-selected');
  icon1.classList.add('styled-button-icon-selected');
});
