"use strict";

Array.from(document.getElementsByClassName('post-vote-icon')).forEach(button => {
  button.addEventListener('click', async event => {
    event.preventDefault();
    const isUpvote = button.classList.contains('post-upvote');
    const postId = button.dataset.postId;
    const counterId = button.dataset.counterId;
    const counter = document.getElementById(counterId);
    if (counter.classList.contains('loading')) return;
    counter.classList.add('loading');
    const upvoteCount = await fetch('/vote/post', {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8',
      },
      body: new URLSearchParams({
        id: postId,
        is_upvote: isUpvote,
        allow_toggle: true,
      }).toString()
    }).then(res => res.text());
    counter.classList.remove('loading');
    //*HACK*
    if (upvoteCount[0] == '<') document.location = '/login?error=You must login before you can vote'; 
    counter.getElementsByClassName('upvctr-value')[0].textContent = (parseInt(upvoteCount) | 0).toString();
  });
});


Array.from(document.getElementsByClassName('post-share-button')).forEach(button => {
  button.addEventListener('click', async () => {
    await shareModal();
  });
})
