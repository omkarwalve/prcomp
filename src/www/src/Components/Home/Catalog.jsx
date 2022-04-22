//    ___   __  ____  __   __     __    ___ 
//   / __) / _\(_  _)/ _\ (  )   /  \  / __)
//  ( (__ /    \ )( /    \/ (_/\(  O )( (_ \
//   \___)\_/\_/(__)\_/\_/\____/ \__/  \___/

// Library Imports
import React,{ useEffect } from 'react';

// File Imports
import { ReactComponent as Electronics } from './assets/elx.svg';
import { ReactComponent as Furniture } from './assets/fur.svg';
import { ReactComponent as Fashion } from './assets/fas.svg';
import { ReactComponent as Grocery } from './assets/gro.svg';
import { ReactComponent as Stationary } from './assets/sta.svg';
import { ReactComponent as Garden } from './assets/gar.svg';
import { ReactComponent as Toys } from './assets/toy.svg';
import { ReactComponent as Books } from './assets/bok.svg';
// Css File
import './Catalog.css';

function Catalog () {
  const catalog = [
    {  
      name: 'Electronics',
      image: <Electronics />
    },
    {  
      name: 'Furniture',
      image: <Furniture />
    },
    {  
      name: 'Fashion',
      image: <Fashion />
    },
    {  
      name: 'Grocery',
      image: <Grocery />
    },
    {  
      name: 'Stationary',
      image: <Stationary />
    },
    {  
      name: 'Garden',
      image: <Garden />
    },
    {  
      name: 'Toys',
      image: <Toys />
    },
    {  
      name: 'Books',
      image: <Books />
    },
  ];
  return(
    <div className="catalog-container">
      <ul className="category-grid-list">
        {
          catalog.map(({name,image}) => {
            return (
              <li className="category-box">
                {image}
                <span className="category-label">{name}</span>
              </li>
            )})
        }
      </ul>
    </div>
  )
}

export default Catalog
