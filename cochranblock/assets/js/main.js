/** Copyright (c) 2026 The Cochran Block. All rights reserved. */
document.querySelectorAll('form').forEach(function(f){
  f.addEventListener('submit',function(){
    var btn=f.querySelector('button[type=submit],input[type=submit]');
    if(btn){btn.disabled=true;btn.textContent='Submitting...'}
  });
});
document.querySelectorAll('.toggle-password').forEach(function(t){
  t.addEventListener('click',function(){
    var i=document.getElementById(t.dataset.target);
    if(i){i.type=i.type==='password'?'text':'password'}
  });
});
