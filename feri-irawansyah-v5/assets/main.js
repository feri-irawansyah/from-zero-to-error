// wasm-loader.js
(function () {
    let reloaded = false;
    caches.keys().then((keys) => console.log("🧹 CacheStorage:", keys));

    async function safeReload(reason, clearCache = false) {
        if (!reloaded) {
            reloaded = true;
            console.warn("🔄 Reloading page karena:", reason);

            if (clearCache) {
                const keys = await caches.keys();
                console.log("🧹 Bersihkan cache:", keys);
                for (const key of keys) {
                    await caches.delete(key);
                }
                console.log("🧹 CacheStorage dibersihkan");
            }

            console.log("🔄 Reloading page...", caches);

            // reload sekali aja
            location.reload();
        }
    }

    // Timeout fallback kalau WASM kelamaan load
    let timeout = setTimeout(() => {
        safeReload("Timeout > 15 detik");
    }, 15000);

    // Observe kalau WASM sukses dimuat
    const observer = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
            if (entry.name.endsWith(".wasm")) {
                console.log("✅ WASM berhasil dimuat:", entry.name);
                clearTimeout(timeout);
                document.getElementById("preloader")?.remove();
                observer.disconnect();
            }
        }
    });
    observer.observe({ type: "resource", buffered: true });

    // Tangkap error jika WASM gagal load
    window.addEventListener("error", (e) => {
        if (e.target && e.target.src && e.target.src.endsWith(".wasm")) {
            safeReload("WASM file gagal dimuat", true);
        }
    }, true);

    // Tangkap panic runtime di WASM (misal unreachable)
    function handleWasmPanic(err) {
        const msg = err?.message || "";
        if (msg.includes("unreachable") || msg.includes("panic")) {
            safeReload("WASM panic runtime", true);
        }
    }

    window.addEventListener("error", (e) => handleWasmPanic(e.error));
    window.addEventListener("unhandledrejection", (ev) => handleWasmPanic(ev.reason));
})();
