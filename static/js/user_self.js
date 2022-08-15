let editing = false;
Array.from(document.querySelectorAll('#user-username,#user-username-mobile')).forEach(elem => {
  elem.addEventListener('click', event => {
    if (editing) return;
    editing = true;
    const old_username = event.currentTarget.textContent;
    const old_element = event.currentTarget;
    const new_element = (() => {
      const elem = document.createElement('div');
      elem.id = "username-editable";
      const input = (() => {
        const wrapper = document.createElement('div');
        const input = document.createElement('input');
        const label = document.createElement('label');
        wrapper.classList.add('styled-input');
        wrapper.append(input);
        wrapper.append(label);
        wrapper.id = elem.id + "-input";
        input.placeholder = " ";
        input.id = wrapper.id + "-inner";
        label.for = input.id;
        label.textContent = old_username;
        input.addEventListener('focus', event => {
          event.currentTarget.value = label.textContent;
        }, { once: true });
        return wrapper;
      })();
      const save_button = (() => {
        const btn = document.createElement('button');
        btn.textContent = "Save";
        btn.id = elem.id + "-submit";
        btn.classList.add('styled-button');
        btn.addEventListener('click', () => {
          editing = false;
          const new_username = input.getElementsByTagName('input')[0].value || old_username;
          old_element.textContent = new_username;
          old_element.classList.add('saving');
          elem.replaceWith(old_element);
          fetch('/update_username', {
            method: 'POST',
            credentials: 'include',
            headers: {
              'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8',
            },
            body: new URLSearchParams({new_username}).toString()
          }).catch(() => {
            old_element.classList.remove('saving');
            old_element.classList.add('save-error');
            old_element.textContent = old_username;
          }).then((res) => {
            old_element.classList.remove('saving');
            if (res.ok) {
              setTimeout(() => document.location.reload(), 100);
            } else {
              old_element.classList.add('save-error');
              old_element.textContent = old_username;
              res.text().then(console.log)
            }
          });
        }, { once: true });
        return btn;
      })();
      elem.append(input, save_button);
      return elem;
    })();
    event.currentTarget.replaceWith(new_element);
  });
});
