document.querySelector("#server-form").addEventListener("submit", (e) => {
  e.preventDefault();

  const left = document.querySelector("#server-left");
  const right = document.querySelector("#server-right");
  const res = document.querySelector("#server-res");

  fetch(`/add?left=${left.value}&right=${right.value}`)
    .then((response) => response.json())
    .then((data) => {
      res.value = data;
    });
});
