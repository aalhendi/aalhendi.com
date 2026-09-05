(() => {
    const requested = new URLSearchParams(window.location.search).get("view");
    const view = ["resume", "highlights"].includes(requested) ? requested : "full";
    document.documentElement.dataset.resumeView = view;
    document.title = view === "full" ? "About - Abdulrazzaq Alhendi" : `Abdulrazzaq Alhendi - ${view === "highlights" ? "Highlights" : "Resume"}`;
})();
