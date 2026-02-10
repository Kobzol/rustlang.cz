(function () {
  const pageLang = document.documentElement.lang;
  const saved = localStorage.getItem("lang");

  if (saved) {
    // User has a saved preference — redirect if on the wrong version
    if (saved !== pageLang) {
      if (saved === "cs") {
        window.location.replace("/cs/");
      } else {
        window.location.replace("/");
      }
    }
  } else if (pageLang === "en") {
    // No preference saved, on the default (English) page — check browser locale
    const browserLang = (navigator.language || "").toLowerCase();
    if (browserLang.startsWith("cs")) {
      window.location.replace("/cs/");
    }
  }
})();