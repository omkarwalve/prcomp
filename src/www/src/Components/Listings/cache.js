//      __   ____     __  __ __    ___ 
//     /  ] /    |   /  ]|  |  |  /  _]
//    /  / |  o  |  /  / |  |  | /  [_ 
//   /  /  |     | /  /  |  _  ||    _]
//  /   \_ |  _  |/   \_ |  |  ||   [_ 
//  \     ||  |  |\     ||  |  ||     |
//   \____||__|__| \____||__|__||_____|
//                                     

// Cache Key for cache r/w from storage
//const STORAGE_KEY = "products-cache";

class ResultsCache {
  constructor(query, result) {
    this.query = query;
    this.result = result;
  }

  static STORAGE_KEY = "products-cache";
  static fetch() {
    let object_arr = JSON.parse(sessionStorage.getItem(ResultsCache.STORAGE_KEY));
    if (object_arr != null && Array(object_arr).length != 0) {
      console.info("Populating runtime cacher with Session Storage cache...");
      return Array(object_arr).filter(object =>
        object != null && object.query != null && object.result != null
        ? true
        : false
      ).map(cached => 
        new ResultsCache(cached.query, cached.result));
    }
    else { console.error("Empty Cacher obtained"); return []; }
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
