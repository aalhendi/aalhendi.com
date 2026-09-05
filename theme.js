(() => {
  const html = document.documentElement;
  const systemTheme = window.matchMedia("(prefers-color-scheme: dark)");
  let savedTheme;
  try {
    savedTheme = localStorage.getItem("theme");
  } catch {
    // Storage can be unavailable; the system preference still works.
  }
  if (savedTheme !== "dark" && savedTheme !== "light") savedTheme = null;

  function applyTheme(theme) {
    html.dataset.theme = theme;
    html.style.colorScheme = theme;
    const toggle = document.getElementById("theme-toggle");
    if (toggle) {
      toggle.textContent = theme === "dark" ? "☀️" : "🌙";
      toggle.setAttribute("aria-label", `Switch to ${theme === "dark" ? "light" : "dark"} mode`);
    }
  }

  // This script runs synchronously in the head, before the first page paint.
  applyTheme(savedTheme || (systemTheme.matches ? "dark" : "light"));

  document.addEventListener("DOMContentLoaded", () => {
    applyTheme(html.dataset.theme);
    document.getElementById("theme-toggle")?.addEventListener("click", () => {
      savedTheme = html.dataset.theme === "dark" ? "light" : "dark";
      applyTheme(savedTheme);
      try {
        localStorage.setItem("theme", savedTheme);
      } catch {
        // Keep the toggle usable even when the browser blocks storage.
      }
    });
  });

  systemTheme.addEventListener("change", () => {
    if (!savedTheme) applyTheme(systemTheme.matches ? "dark" : "light");
  });
})();
