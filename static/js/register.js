const icon0 = document.getElementById("reg-icon-cont");
const icon1 = document.getElementById("reg-icon-loading");
const form = document.getElementById("reg-form");
const subm = document.getElementById("reg-submit-btn");

form.addEventListener('submit', () => {
  subm.disabled = true;
  icon0.classList.remove('styled-button-icon-selected');
  icon1.classList.add('styled-button-icon-selected');
});
