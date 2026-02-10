(function () {
  const pageLang = document.documentElement.lang;
  const saved = localStorage.getItem("lang");

  if (!saved) {
    // No preference saved yet — detect from browser locale
    const browserLang = (navigator.language || "").toLowerCase();
    const preferred = browserLang.startsWith("cs") ? "cs" : "en";

    if (preferred !== pageLang) {
      // Redirect to the preferred language version
      if (preferred === "cs") {
        window.location.replace("/cs/");
      } else {
        window.location.replace("/");
      }
    }
  }
})();
