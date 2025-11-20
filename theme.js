(function () {
  const themeToggle = document.getElementById("theme-toggle");
  const html = document.documentElement;

  const currentTheme =
    localStorage.getItem("theme") ||
    (window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light");

  if (currentTheme === "dark") {
    html.setAttribute("data-theme", "dark");
    themeToggle.textContent = "☀️";
  } else {
    html.setAttribute("data-theme", "light");
    themeToggle.textContent = "🌙";
  }

  if (themeToggle) {
    themeToggle.addEventListener("click", function () {
      const isDark = html.getAttribute("data-theme") === "dark";

      if (isDark) {
        html.setAttribute("data-theme", "light");
        themeToggle.textContent = "🌙";
        localStorage.setItem("theme", "light");
      } else {
        html.setAttribute("data-theme", "dark");
        themeToggle.textContent = "☀️";
        localStorage.setItem("theme", "dark");
      }
    });
  }
})();
