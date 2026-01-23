// @description Get cookie by name
function getCookie(name) {
  return document.cookie
    .split("; ")
    .find(row => row.startsWith(name + "="))
    ?.split("=")[1];
}

const DEFAULT_THEME = 'light'

window.addEventListener('load', (e) => {
  let theme = getCookie('theme');

  if (theme) {
    // change theme
    document.querySelector('html').setAttribute("data-bs-theme", theme);
    // update value of input
    document.querySelector("#changeTheme").value = theme
  } else {
    document.querySelector('html').setAttribute("data-bs-theme", DEFAULT_THEME);
  }
});

document.querySelector("#changeTheme").addEventListener('change', (e) => {
  // create cookie to save theme preferences
  document.cookie = `theme=${e.target.value}; path=/; max-age=31536000`;
  //change theme
  document.querySelector('html').setAttribute("data-bs-theme", e.target.value);
});
