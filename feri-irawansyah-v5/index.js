// static/js/utils.js
export function initAOS() {
    AOS.init({
        disable: false,
        startEvent: "DOMContentLoaded",
        initClassName: "aos-init",
        animatedClassName: "aos-animate",
        useClassNames: false,
        disableMutationObserver: false,
        debounceDelay: 50,
        throttleDelay: 99,
        offset: -9999,
        delay: 0,
        duration: 600,
        easing: "ease",
        once: false,
        mirror: false,
        anchorPlacement: "top-center",
    });
}

export function refreshAOS() {
    AOS.refresh();
}

export function openModal(modal_id) {
    const modal = new bootstrap.Modal(document.getElementById(modal_id));
    modal.show();
}

export function closeModal(modal_id) {
    const modal = new bootstrap.Modal(document.getElementById(modal_id));
    modal.hide();
}

const STORAGE_KEY = "search_history";

export function saveSearch(searchItem) {
  try {
    // Ambil yang lama
    const raw = localStorage.getItem(STORAGE_KEY);
    let history = raw ? JSON.parse(raw) : [];

    // Tambah item baru di depan
    history.unshift(searchItem);

    // Batasi max 20 item misalnya
    if (history.length > 20) {
      history = history.slice(0, 20);
    }

    // Simpan lagi
    localStorage.setItem(STORAGE_KEY, JSON.stringify(history));
  } catch (err) {
    console.error("Gagal simpan search history:", err);
  }
}

export function deleteSearch(term, url) {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    let history = raw ? JSON.parse(raw) : [];

    history = history.filter((item) => !(item.term === term && item.url === url));

    localStorage.setItem(STORAGE_KEY, JSON.stringify(history));
    console.log("History setelah hapus:", history);
  } catch (err) {
    console.error("Gagal hapus search history:", err);
  }
}

export function getSearch() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? JSON.parse(raw) : [];
  } catch (err) {
    console.error("Gagal ambil search history:", err);
    return [];
  }
}
