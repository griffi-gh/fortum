if (document.getElementById("header")) {
  const ups = document.getElementById("user-profile-section");
  const lob = document.getElementById("logout-button");
  lob.addEventListener('click', event => {
    event.stopPropagation();
  })
  ups.addEventListener('click', event => {
    event.preventDefault();
  });
} 
