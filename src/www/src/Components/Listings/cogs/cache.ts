//      __   ____     __  __ __    ___ 
//     /  ] /    |   /  ]|  |  |  /  _]
//    /  / |  o  |  /  / |  |  | /  [_ 
//   /  /  |     | /  /  |  _  ||    _]
//  /   \_ |  _  |/   \_ |  |  ||   [_ 
//  \     ||  |  |\     ||  |  ||     |
//   \____||__|__| \____||__|__||_____|
//                                     

import Product from './product';

class ResultsCache {
  query : string;
  result : Product[];
  constructor(query: string, result: Product[]) {
    this.query = query;
    this.result = result;
  }

  static #STORAGE_KEY = "products-cache";
  /**
   * Retrieves [ResultsCache] from the browser storage
// * @return {[ResultsCache] | []}  All the cache from storage
   */
  static retrieve() {
    let cache_in_storage = sessionStorage.getItem(ResultsCache.#STORAGE_KEY);
    if (cache_in_storage) {
      let object_arr : ResultsCache[] | null = JSON.parse(cache_in_storage);
      console.log(object_arr);
      if (object_arr && object_arr.length) {
        console.info("Obtaining cache from storage...");
        return object_arr.filter(object =>
          object != null && object.query != null && object.result != null
          ? true
          : false
        ).map(cached => 
          new ResultsCache(cached.query, cached.result));
      } else { console.error("Error resolving cache to ResultsCache found in storage"); return []; }
    } else { console.error("No cache found in storage"); return []; }
  }

  static store(runtime_cache : ResultsCache[]) {
    runtime_cache && runtime_cache.length 
      ? sessionStorage.setItem(ResultsCache.#STORAGE_KEY,JSON.stringify(runtime_cache))
      //: console.error("Runtime cache empty :(");
      : console.error("Skipped caching of rc :(");
  }

  static filter(cacher : ResultsCache[], query : string) {
    const match = cacher.filter(cache =>
      //(cache && cache != [] && cache.length)
      (cache)
       ? cache.is(query)
       : false
    );
    return (match.length 
            ? match
            : null)
  }

  is(search_query: string) {
    if (this.query === search_query.trim().toLowerCase()) {
      return true;
    }
  }

  stringify() {
    return "{ query: " + this.query + ", results: [" + this.result + "] }" ;
  }
}

export default ResultsCache
