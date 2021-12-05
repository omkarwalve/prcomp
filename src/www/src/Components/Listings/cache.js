//      __   ____     __  __ __    ___ 
//     /  ] /    |   /  ]|  |  |  /  _]
//    /  / |  o  |  /  / |  |  | /  [_ 
//   /  /  |     | /  /  |  _  ||    _]
//  /   \_ |  _  |/   \_ |  |  ||   [_ 
//  \     ||  |  |\     ||  |  ||     |
//   \____||__|__| \____||__|__||_____|
//                                     

class ResultsCache {
  constructor(query, result) {
    this.query = query;
    this.result = result;
  }

  static #STORAGE_KEY = "products-cache";
  /**
   * Retrieves [ResultsCache] from the browser storage
   * @return {[ResultsCache]} All the cache from storage
   */
  static retrieve() {
    let object_arr = JSON.parse(sessionStorage.getItem(ResultsCache.#STORAGE_KEY));
    if (object_arr != null && Array(object_arr).length != 0) {
      console.info("Obtaining cache from storage...");
      return Array(object_arr).filter(object =>
        object != null && object.query != null && object.result != null
        ? true
        : false
      ).map(cached => 
        new ResultsCache(cached.query, cached.result));
    }
    else { console.error("No cache found in storage"); return []; }
  }

  static store(runtime_cache) {
    runtime_cache.length 
      ? sessionStorage.setItem(ResultsCache.#STORAGE_KEY,JSON.stringify(runtime_cache))
      : console.error("Runtime cache empty :(");
  }

  static filter(cacher, query) {
    return cacher.filter(cache =>
      (cache != null && cache != [] && cache.length != 0)
       ? cache.is(query)
       : false
    );
  }

  is(search_query) {
    if (this.query === search_query.trim().toLowerCase()) {
      return true;
    }
  }

  stringify() {
    return "{ query: " + this.query + ", results: [" + this.result + "] }" ;
  }
}

export default ResultsCache
