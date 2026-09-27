(() => {
  const AD_KEYS = ["adPlacements", "adSlots", "playerAds"];

  const prune = (value) => {
    if (!value || typeof value !== "object") {
      return value;
    }
    for (const key of AD_KEYS) {
      delete value[key];
    }
    if (value.playerResponse && typeof value.playerResponse === "object") {
      prune(value.playerResponse);
    }
    return value;
  };

  let initialPlayerResponse;
  Object.defineProperty(window, "ytInitialPlayerResponse", {
    configurable: true,
    get: () => initialPlayerResponse,
    set: (value) => {
      initialPlayerResponse = prune(value);
    },
  });

  const parse = JSON.parse;
  JSON.parse = function (...args) {
    return prune(parse.apply(this, args));
  };

  const json = Response.prototype.json;
  Response.prototype.json = function (...args) {
    return json.apply(this, args).then(prune);
  };
})();
