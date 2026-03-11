/** Copyright (c) 2026 The Cochran Block. All rights reserved. */
(function() {
  'use strict';

  var WEEKDAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  var MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];
  var HOURS = ['12am', '1am', '2am', '3am', '4am', '5am', '6am', '7am', '8am', '9am', '10am', '11am', '12pm', '1pm', '2pm', '3pm', '4pm', '5pm', '6pm', '7pm', '8pm', '9pm', '10pm', '11pm'];

  function parseSlots() {
    var el = document.getElementById('booking-slots-data');
    if (!el || !el.textContent) return [];
    try {
      return JSON.parse(el.textContent);
    } catch (e) {
      return [];
    }
  }

  function dateKey(year, month, day) {
    var m = String(month + 1);
    var d = String(day);
    if (m.length < 2) m = '0' + m;
    if (d.length < 2) d = '0' + d;
    return year + '-' + m + '-' + d;
  }

  function buildAvailableSet(slots) {
    var set = {};
    for (var i = 0; i < slots.length; i++) {
      set[slots[i].date] = slots[i];
    }
    return set;
  }

  function parseDateKey(key) {
    var parts = key.split('-');
    return { year: parseInt(parts[0], 10), month: parseInt(parts[1], 10) - 1, day: parseInt(parts[2], 10) };
  }

  function getWeekStart(d) {
    var day = d.getDay();
    var diff = d.getDate() - day;
    return new Date(d.getFullYear(), d.getMonth(), diff);
  }

  function addDays(d, n) {
    var r = new Date(d);
    r.setDate(r.getDate() + n);
    return r;
  }

  function isSameDay(a, b) {
    return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate();
  }

  function formatDateShort(d) {
    return MONTHS[d.getMonth()] + ' ' + d.getDate() + ', ' + d.getFullYear();
  }

  function formatWeekRange(start) {
    var end = addDays(start, 6);
    return MONTHS[start.getMonth()] + ' ' + start.getDate() + ' – ' + MONTHS[end.getMonth()] + ' ' + end.getDate() + ', ' + start.getFullYear();
  }

  /* --- Month view --- */
  function renderMonth(gridEl, year, month, available, today, selectedKey, onSelect) {
    var first = new Date(year, month, 1);
    var last = new Date(year, month + 1, 0);
    var startDow = first.getDay();
    var daysInMonth = last.getDate();
    var todayKey = dateKey(today.getFullYear(), today.getMonth(), today.getDate());

    gridEl.innerHTML = '';
    gridEl.setAttribute('aria-label', MONTHS[month] + ' ' + year);

    for (var d = 0; d < WEEKDAYS.length; d++) {
      var th = document.createElement('div');
      th.className = 'gcal-weekday';
      th.setAttribute('role', 'columnheader');
      th.textContent = WEEKDAYS[d];
      gridEl.appendChild(th);
    }

    var pad = startDow;
    for (var p = 0; p < pad; p++) {
      var empty = document.createElement('div');
      empty.className = 'gcal-day gcal-day-empty';
      empty.setAttribute('aria-hidden', 'true');
      gridEl.appendChild(empty);
    }

    for (var day = 1; day <= daysInMonth; day++) {
      var key = dateKey(year, month, day);
      var slot = available[key];
      var cell = document.createElement('div');
      cell.className = 'gcal-day';
      cell.setAttribute('role', 'gridcell');
      cell.setAttribute('data-date', key);

      var btn = document.createElement('button');
      btn.type = 'button';
      btn.className = 'gcal-day-btn';
      btn.textContent = day;
      btn.setAttribute('aria-label', 'Select ' + MONTHS[month] + ' ' + day + (slot ? ', available' : ''));

      if (slot) {
        btn.classList.add('gcal-day-available');
        btn.addEventListener('click', (function(k, s) { return function() { onSelect(k, s); }; })(key, slot));
      } else {
        btn.classList.add('gcal-day-unavailable');
        btn.disabled = true;
      }

      if (key === todayKey) btn.classList.add('gcal-day-today');
      if (key === selectedKey) cell.classList.add('gcal-day-selected');

      cell.appendChild(btn);
      gridEl.appendChild(cell);
    }
  }

  /* --- Week view --- */
  function renderWeek(containerEl, weekStart, available, today, onSelect) {
    containerEl.innerHTML = '';
    var timeCol = document.createElement('div');
    timeCol.className = 'gcal-week-time-col';
    var header = document.createElement('div');
    header.className = 'gcal-week-time-header';
    header.textContent = '';
    timeCol.appendChild(header);
    for (var h = 8; h <= 17; h++) {
      var row = document.createElement('div');
      row.className = 'gcal-week-time-slot';
      row.textContent = HOURS[h];
      timeCol.appendChild(row);
    }
    containerEl.appendChild(timeCol);

    for (var d = 0; d < 7; d++) {
      var dayDate = addDays(weekStart, d);
      var key = dateKey(dayDate.getFullYear(), dayDate.getMonth(), dayDate.getDate());
      var slot = available[key];
      var col = document.createElement('div');
      col.className = 'gcal-week-day-col';
      col.setAttribute('data-date', key);

      var dayHeader = document.createElement('div');
      dayHeader.className = 'gcal-week-day-header';
      dayHeader.innerHTML = '<span class="gcal-week-day-name">' + WEEKDAYS[d] + '</span><span class="gcal-week-day-num">' + dayDate.getDate() + '</span>';
      if (slot) {
        dayHeader.classList.add('gcal-week-day-available');
        dayHeader.addEventListener('click', (function(k, s) { return function() { onSelect(k, s); }; })(key, slot));
      }
      col.appendChild(dayHeader);

      for (var h = 8; h <= 17; h++) {
        var cell = document.createElement('div');
        cell.className = 'gcal-week-cell';
        if (slot && h >= 8 && h < 18 && slot.times) {
          cell.classList.add('gcal-week-cell-available');
          var startIdx = (h - 8) * 2;
          for (var si = 0; si < 2 && slot.times[startIdx + si]; si++) {
            var t = slot.times[startIdx + si];
            var link = document.createElement('a');
            link.href = t.mailto;
            link.className = 'gcal-week-slot-link';
            link.textContent = t.label;
            link.setAttribute('rel', 'noopener');
            cell.appendChild(link);
          }
        }
        col.appendChild(cell);
      }
      containerEl.appendChild(col);
    }
  }

  /* --- Day view --- */
  function renderDay(containerEl, date, available, today, onSelect) {
    containerEl.innerHTML = '';
    var key = dateKey(date.getFullYear(), date.getMonth(), date.getDate());
    var slot = available[key];

    var timeCol = document.createElement('div');
    timeCol.className = 'gcal-day-time-col';
    for (var h = 8; h <= 17; h++) {
      var row = document.createElement('div');
      row.className = 'gcal-day-time-slot';
      row.textContent = HOURS[h];
      timeCol.appendChild(row);
    }
    containerEl.appendChild(timeCol);

    var mainCol = document.createElement('div');
    mainCol.className = 'gcal-day-main-col';
    var dayHeader = document.createElement('div');
    dayHeader.className = 'gcal-day-header';
    dayHeader.textContent = WEEKDAYS[date.getDay()] + ', ' + MONTHS[date.getMonth()] + ' ' + date.getDate() + ', ' + date.getFullYear();
    if (slot) {
      dayHeader.classList.add('gcal-day-available');
      dayHeader.addEventListener('click', (function(k, s) { return function() { onSelect(k, s); }; })(key, slot));
    }
    mainCol.appendChild(dayHeader);

      for (var h = 8; h <= 17; h++) {
      var cell = document.createElement('div');
      cell.className = 'gcal-day-cell';
      if (slot && h >= 8 && h < 18 && slot.times) {
        cell.classList.add('gcal-day-cell-available');
        var startIdx = (h - 8) * 2;
        for (var si = 0; si < 2 && slot.times[startIdx + si]; si++) {
          var t = slot.times[startIdx + si];
          var link = document.createElement('a');
          link.href = t.mailto;
          link.className = 'gcal-day-slot-link';
          link.textContent = t.label;
          link.setAttribute('rel', 'noopener');
          cell.appendChild(link);
        }
      }
      mainCol.appendChild(cell);
    }
    containerEl.appendChild(mainCol);
  }

  /* --- Agenda view --- */
  function renderAgenda(containerEl, slots, onSelect) {
    containerEl.innerHTML = '';
    if (!slots || slots.length === 0) {
      containerEl.innerHTML = '<p class="gcal-agenda-empty">No upcoming availability. <a href="mailto:mclarkfyrue@gmail.com?subject=Discovery%20Call%20Request">Email to propose a time</a>.</p>';
      return;
    }
    for (var i = 0; i < slots.length; i++) {
      var s = slots[i];
      var item = document.createElement('div');
      item.className = 'gcal-agenda-item';
      item.setAttribute('data-date', s.date);
      var dateEl = document.createElement('div');
      dateEl.className = 'gcal-agenda-date';
      dateEl.textContent = s.day_name + ', ' + s.date_label;
      item.appendChild(dateEl);
      var timesEl = document.createElement('div');
      timesEl.className = 'gcal-agenda-times';
      for (var t = 0; t < s.times.length; t++) {
        var a = document.createElement('a');
        a.href = s.times[t].mailto;
        a.className = 'gcal-agenda-slot';
        a.textContent = s.times[t].label;
        a.setAttribute('rel', 'noopener');
        timesEl.appendChild(a);
      }
      item.appendChild(timesEl);
      containerEl.appendChild(item);
    }
  }

  /* --- Time panel (shared) --- */
  function showTimePanel(slot) {
    var panel = document.getElementById('booking-time-panel');
    var heading = document.getElementById('booking-time-heading');
    var slotsEl = document.getElementById('booking-time-slots');
    if (!panel || !heading || !slotsEl) return;

    heading.textContent = 'Pick a time for ' + slot.day_name + ', ' + slot.date_label;
    slotsEl.innerHTML = '';
    for (var i = 0; i < slot.times.length; i++) {
      var t = slot.times[i];
      var a = document.createElement('a');
      a.href = t.mailto;
      a.className = 'booking-time-slot';
      a.textContent = t.label;
      a.setAttribute('rel', 'noopener');
      a.setAttribute('role', 'button');
      slotsEl.appendChild(a);
    }
    panel.hidden = false;
    panel.setAttribute('aria-hidden', 'false');
    panel.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
  }

  function clearSelection() {
    var grid = document.getElementById('gcal-month-grid');
    if (grid) {
      var prev = grid.querySelector('.gcal-day-selected');
      if (prev) prev.classList.remove('gcal-day-selected');
    }
  }

  function selectDate(dateKey, slot) {
    clearSelection();
    var cell = document.querySelector('[data-date="' + dateKey + '"]');
    if (cell) cell.classList.add('gcal-day-selected');
    showTimePanel(slot);
  }

  /* --- Mini calendar --- */
  function renderMiniCalendar(containerEl, year, month, available, today, onDateClick) {
    if (!containerEl) return;
    var first = new Date(year, month, 1);
    var startDow = first.getDay();
    var last = new Date(year, month + 1, 0);
    var daysInMonth = last.getDate();
    var todayKey = dateKey(today.getFullYear(), today.getMonth(), today.getDate());

    containerEl.innerHTML = '';
    for (var d = 0; d < 7; d++) {
      var wd = document.createElement('div');
      wd.className = 'gcal-mini-weekday';
      wd.textContent = WEEKDAYS[d].charAt(0);
      containerEl.appendChild(wd);
    }
    for (var p = 0; p < startDow; p++) {
      var empty = document.createElement('div');
      empty.className = 'gcal-mini-day';
      empty.setAttribute('aria-hidden', 'true');
      containerEl.appendChild(empty);
    }
    for (var day = 1; day <= daysInMonth; day++) {
      var key = dateKey(year, month, day);
      var slot = available[key];
      var cell = document.createElement('button');
      cell.type = 'button';
      cell.className = 'gcal-mini-day';
      cell.textContent = day;
      cell.setAttribute('aria-label', 'Go to ' + MONTHS[month] + ' ' + day);
      if (key === todayKey) cell.classList.add('today');
      if (slot) cell.classList.add('available');
      cell.addEventListener('click', (function(k, y, m, d) {
        return function() { onDateClick(k, y, m, d); };
      })(key, year, month, day - 1));
      containerEl.appendChild(cell);
    }
  }

  /* --- Current time indicator --- */
  function addNowIndicator(containerEl) {
    if (!containerEl) return;
    var parent = containerEl.parentElement;
    if (!parent) return;
    var existing = parent.querySelector('.gcal-now-indicator');
    if (existing) existing.remove();
    var now = new Date();
    var hour = now.getHours() + now.getMinutes() / 60;
    if (hour < 8 || hour > 17) return;
    var headerHeight = 36;
    var rowHeight = 48;
    var topPx = headerHeight + (hour - 8) * rowHeight;
    var line = document.createElement('div');
    line.className = 'gcal-now-indicator';
    line.style.top = topPx + 'px';
    line.style.left = '48px';
    line.style.right = '0';
    parent.style.position = 'relative';
    parent.appendChild(line);
  }

  /* --- Main init --- */
  function init() {
    var slots = parseSlots();
    var available = buildAvailableSet(slots);

    var viewMonth = document.getElementById('gcal-view-month');
    var viewWeek = document.getElementById('gcal-view-week');
    var viewDay = document.getElementById('gcal-view-day');
    var viewAgenda = document.getElementById('gcal-view-agenda');
    var todayBtn = document.getElementById('gcal-today');
    var prevBtn = document.getElementById('gcal-prev');
    var nextBtn = document.getElementById('gcal-next');
    var dateLabel = document.getElementById('gcal-date-label');
    var monthPane = document.getElementById('gcal-month-pane');
    var weekPane = document.getElementById('gcal-week-pane');
    var dayPane = document.getElementById('gcal-day-pane');
    var agendaPane = document.getElementById('gcal-agenda-pane');
    var monthGrid = document.getElementById('gcal-month-grid');
    var weekContainer = document.getElementById('gcal-week-container');
    var dayContainer = document.getElementById('gcal-day-container');
    var agendaContainer = document.getElementById('gcal-agenda-container');
    var sidebar = document.getElementById('gcal-sidebar');
    var sidebarToggle = document.getElementById('gcal-sidebar-toggle');
    var miniGrid = document.getElementById('gcal-mini-grid');
    var miniHeader = document.getElementById('gcal-mini-month');

    if (!monthPane || !weekPane || !dayPane || !agendaPane) return;

    var now = new Date();
    var currentView = 'month';
    var viewDate = new Date(now);
    var viewYear = now.getFullYear();
    var viewMonth = now.getMonth();
    var weekStart = getWeekStart(now);

    function setView(v) {
      currentView = v;
      var views = ['month', 'week', 'day', 'agenda'];
      var btns = [viewMonth, viewWeek, viewDay, viewAgenda];
      var panes = [monthPane, weekPane, dayPane, agendaPane];
      for (var i = 0; i < 4; i++) {
        var active = views[i] === v;
        if (btns[i]) {
          btns[i].classList.toggle('active', active);
          btns[i].setAttribute('aria-selected', active ? 'true' : 'false');
        }
        if (panes[i]) {
          panes[i].classList.toggle('gcal-pane-active', active);
          panes[i].setAttribute('aria-hidden', active ? 'false' : 'true');
          if (active && panes[i]) panes[i].focus();
        }
      }
      update();
    }

    function goToDate(key, year, month, day) {
      viewYear = year;
      viewMonth = month;
      viewDate = new Date(year, month, day);
      weekStart = getWeekStart(viewDate);
      update();
    }

    function update() {
      if (currentView === 'month') {
        dateLabel.textContent = MONTHS[viewMonth] + ' ' + viewYear;
        if (monthGrid) renderMonth(monthGrid, viewYear, viewMonth, available, now, null, selectDate);
      } else if (currentView === 'week') {
        dateLabel.textContent = formatWeekRange(weekStart);
        if (weekContainer) {
          renderWeek(weekContainer, weekStart, available, now, selectDate);
          var weekEnd = addDays(weekStart, 6);
          if (now >= weekStart && now <= weekEnd) addNowIndicator(weekContainer);
        }
      } else if (currentView === 'day') {
        dateLabel.textContent = formatDateShort(viewDate);
        if (dayContainer) {
          renderDay(dayContainer, viewDate, available, now, selectDate);
          if (isSameDay(viewDate, now)) addNowIndicator(dayContainer);
        }
      } else {
        dateLabel.textContent = 'Upcoming';
        if (agendaContainer) renderAgenda(agendaContainer, slots, selectDate);
      }
      if (miniGrid && miniHeader) {
        miniHeader.textContent = MONTHS[viewMonth].substring(0, 3) + ' ' + viewYear;
        renderMiniCalendar(miniGrid, viewYear, viewMonth, available, now, goToDate);
      }
    }

    if (todayBtn) {
      todayBtn.addEventListener('click', function() {
        now = new Date();
        viewDate = new Date(now);
        viewYear = now.getFullYear();
        viewMonth = now.getMonth();
        weekStart = getWeekStart(now);
        update();
      });
    }

    if (prevBtn) {
      prevBtn.addEventListener('click', function() {
        if (currentView === 'month') {
          viewMonth--;
          if (viewMonth < 0) { viewMonth = 11; viewYear--; }
        } else if (currentView === 'week') {
          weekStart = addDays(weekStart, -7);
        } else if (currentView === 'day') {
          viewDate = addDays(viewDate, -1);
        }
        update();
      });
    }

    if (nextBtn) {
      nextBtn.addEventListener('click', function() {
        if (currentView === 'month') {
          viewMonth++;
          if (viewMonth > 11) { viewMonth = 0; viewYear++; }
        } else if (currentView === 'week') {
          weekStart = addDays(weekStart, 7);
        } else if (currentView === 'day') {
          viewDate = addDays(viewDate, 1);
        }
        update();
      });
    }

    if (viewMonth) viewMonth.addEventListener('click', function() { setView('month'); });
    if (viewWeek) viewWeek.addEventListener('click', function() { setView('week'); });
    if (viewDay) viewDay.addEventListener('click', function() { setView('day'); });
    if (viewAgenda) viewAgenda.addEventListener('click', function() { setView('agenda'); });

    if (sidebarToggle && sidebar) {
      sidebarToggle.addEventListener('click', function() {
        var collapsed = sidebar.classList.toggle('collapsed');
        sidebarToggle.setAttribute('aria-expanded', collapsed ? 'false' : 'true');
      });
    }

    document.addEventListener('keydown', function(e) {
      if (e.target && (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA')) return;
      if (e.key === 't' || e.key === 'T') {
        if (todayBtn) { todayBtn.click(); e.preventDefault(); }
      }
    });

    setView('month');
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
