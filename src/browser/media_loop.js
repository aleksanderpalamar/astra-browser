(() => {
  const LOOP_MARGIN_SECONDS = 0.35;

  const restartBeforeEnd = (event) => {
    const video = event.target;
    if (!(video instanceof HTMLVideoElement) || !video.loop || !video.currentSrc.startsWith("blob:")) {
      return;
    }
    if (video.duration - video.currentTime < LOOP_MARGIN_SECONDS) {
      video.currentTime = 0;
    }
  };

  document.addEventListener("timeupdate", restartBeforeEnd, true);
})();
