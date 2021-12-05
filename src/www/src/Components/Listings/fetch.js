

// Vars
const SERVER_URL = 'http://localhost:8000';

// # Timeout for JS Fetch API
const Timeout = (time) => {
     let controller = new AbortController();
     setTimeout(() => controller.abort(), time * 1000)
     return controller;
}

class Fetch {
  static async get(query) {
  }

  timeout(time) {
     let controller = new AbortController();
     setTimeout(() => controller.abort(), time * 1000)
     return controller;
  }
}

export default Fetch
