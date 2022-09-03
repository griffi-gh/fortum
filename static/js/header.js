// const logoutButton = document.getElementById('logout-button');
// if (logoutButton) {
//   logoutButton.for = "";
//   logoutButton.addEventListener('click', async event => {
//     event.preventDefault();
//     event.stopPropagation();
//     await fetch('/logout', {
//       method: 'POST',
//       credentials: 'include'
//     });
//     console.log('Logged out');
//     document.location = '/';
//   });
// }

// const popupState = document.getElementById('state-header-popup');
// document.addEventListener('click', event => {
//   if (!popupState.checked) return;
//   const closest = event.target.closest('#header-popup, #header, #state-header-popup');
//   if (closest == null) {
//     popupState.checked = false;
//     console.log('blur overlay');
//   }
// });
