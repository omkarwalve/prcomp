//  .____     .__           __   .__                         __                 
//  |    |    |__|  _______/  |_ |__|  ____    ____         |__|  _________  ___
//  |    |    |  | /  ___/\   __\|  | /    \  / ___\        |  | /  ___/\  \/  /
//  |    |___ |  | \___ \  |  |  |  ||   |  \/ /_/  >       |  | \___ \  >    < 
//  |_______ \|__|/____  > |__|  |__||___|  /\___  / /\ /\__|  |/____  >/__/\_ \
//          \/         \/                 \//_____/  \/ \______|     \/       \/

// Library Imports
import React, { useEffect, useState} from 'react';
import { useLocation } from "react-router-dom";

// File Imports
import ResultsCache from './cache';
import Fetch from './fetch';

// Custom Hook
function useQuery() {
    return new URLSearchParams(useLocation().search);
}


const Listing = () => {
  let query = useQuery();
  const category = query.get('cat');
  const search = query.get('search');
  document.title = "kilowog(" + category + ") =>[" + search + "]";

  const [cache,setCache] = useState(ResultsCache.retrieve());
  const [products,setProducts] = useState([]);
  const [loading,setLoading] = useState(false);
  const [crashed,setCrashed] = useState(false);

  useEffect(()=> {
    Fetch.cGET(
      category,
      search,
      {
        cache: cache,
        crashed: setCrashed,
        loading: setLoading,
        products: setProducts,
        setCache: setCache,
      }
    )
  }, [category,search]);

  useEffect(()=> {
    ResultsCache.store(cache);
  },[cache]);

  useEffect(()=> {
    console.log(products);
  },[products]);

  return (
    <div className="product-section">
    </div>
  )
}

export default Listing
