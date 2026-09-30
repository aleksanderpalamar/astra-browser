const LOOP_MARGIN_SECONDS = 0.35;

const loopRestartTime = ({ loop, mediaSource, paused, seeking, duration, currentTime, playbackRate, seekableStart }) => {
  if (!loop || !mediaSource || paused || seeking || !Number.isFinite(duration)) {
    return null;
  }
  const margin = LOOP_MARGIN_SECONDS * Math.max(1, playbackRate);
  if (duration - seekableStart <= 2 * margin) {
    return null;
  }
  return duration - currentTime < margin ? seekableStart : null;
};
