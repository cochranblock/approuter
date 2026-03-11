// Bridge: pass scroll and mouse from JS into mural-wasm
(function() {
  if (typeof miniquad_add_plugin !== 'function') {
    console.error('mural-bridge: miniquad_add_plugin not found. Load gl.js first.');
    return;
  }
  miniquad_add_plugin({
    on_init: function() {
      setInterval(function() {
        try {
          if (wasm_exports && wasm_exports.mural_set_scroll_x) wasm_exports.mural_set_scroll_x(window.scrollX);
          if (wasm_exports && wasm_exports.mural_set_scroll_y) wasm_exports.mural_set_scroll_y(window.scrollY);
        } catch (e) { console.warn('mural-bridge scroll:', e); }
      }, 50);
      var c = document.getElementById('glcanvas');
      if (c && wasm_exports && typeof wasm_exports.mural_set_mouse === 'function') {
        c.addEventListener('mousemove', function(e) {
          try {
            var r = c.getBoundingClientRect();
            var w = r.width || 1, h = r.height || 1;
            var cw = c.width || 1, ch = c.height || 1;
            wasm_exports.mural_set_mouse((e.clientX - r.left) * (cw / w), (e.clientY - r.top) * (ch / h));
          } catch (err) { console.warn('mural-bridge mouse:', err); }
        });
      }
    }
  });
})();
