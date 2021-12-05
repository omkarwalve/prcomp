
import ResultsCache from './cache';

/**
 * Assigns a value to the property if the property exists for the object.
 * Else logs an error to the console.
 * @param {object} object - The object to check in
 * @param {string} property - The property to check for
 * @param {any} value - The value to set
 */
function setIfHas(object,property,value) {
   object.hasOwnProperty(property) 
    ? object.property(value) 
    : console.error(`${property} not found in ${object}`);
 }

class Fetch {
  static #SERVER_URL = 'http://localhost:8000';

  /**
   * Cache aware GET to backend server and update the setters.
   * If query is found in cache then no GET is made to server instead setter is updated with cached data
   * @param {string} category - The chosen category
   * @param {string} query - The search query (with spaces)
   * @param {object} setterz - The object containing all the setters update on different stages of GET.
   */
  static async cGET(category,query,setterz) {
    //setterz.hasOwnProperty('loading') ? setterz.loading(true) : console.error("Missing loading setState!");
    setterz.cache + setterz.setCache ;
  }

  /**
   * GET to backend server and update the setters (cache unaware).
   * @param {string} category - The chosen category
   * @param {string} query - The search query (with spaces)
   * @param {object} setterz - The object containing all the setters update on different stages of GET.
   */
  static async GET(category,query,setterz) {
    setIfHas(setterz,'loading',true);
    try {
    const response = await fetch(`${Fetch.#SERVER_URL}/${category}/${query.split(/\s+/).join('+')}`
                                , { signal: Fetch.#timeout(15).signal })
                                .then(response => response.json())
    } catch(err) { 
      setIfHas(setterz,'loading',false); 
      setIfHas(setterz,'crashed',true); 
    }

    response?.listings 
    ? setIfHas(setterz,'products',response?.listings)
    : setIfHas(setterz,'crashed',true)
  }

  static #timeout(time) {
     let controller = new AbortController();
     setTimeout(() => controller.abort(), time * 1000)
     return controller;
  }
}

export default Fetch
