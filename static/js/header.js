const logoutButton = document.getElementById('logout-button');
if (logoutButton) {
  logoutButton.for = "";
  logoutButton.addEventListener('click', async event => {
    event.preventDefault();
    event.stopPropagation();
    await fetch('/logout', {
      method: 'POST',
      credentials: 'include'
    });
    console.log('Logged out');
    document.location = '/';
  });
}
