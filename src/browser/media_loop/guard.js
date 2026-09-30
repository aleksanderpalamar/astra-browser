const mediaSourceUrls = new Set();

const isMediaSource = (object) => typeof MediaSource !== "undefined" && object instanceof MediaSource;

const createObjectURL = URL.createObjectURL;
URL.createObjectURL = function (object) {
  const url = createObjectURL.call(this, object);
  if (isMediaSource(object)) {
    mediaSourceUrls.add(url);
  }
  return url;
};

const usesMediaSource = (video) => mediaSourceUrls.has(video.currentSrc) || isMediaSource(video.srcObject);

const playbackState = (video) => ({
  loop: video.loop,
  mediaSource: usesMediaSource(video),
  paused: video.paused,
  seeking: video.seeking,
  duration: video.duration,
  currentTime: video.currentTime,
  playbackRate: video.playbackRate,
  seekableStart: video.seekable.length > 0 ? video.seekable.start(0) : 0,
});

const restartBeforeEnd = (event) => {
  const video = event.target;
  if (!(video instanceof HTMLVideoElement)) {
    return;
  }
  const restartTime = loopRestartTime(playbackState(video));
  if (restartTime !== null) {
    video.currentTime = restartTime;
  }
};

const attachShadow = Element.prototype.attachShadow;
Element.prototype.attachShadow = function (...args) {
  const root = attachShadow.apply(this, args);
  root.addEventListener("timeupdate", restartBeforeEnd, true);
  return root;
};

document.addEventListener("timeupdate", restartBeforeEnd, true);
