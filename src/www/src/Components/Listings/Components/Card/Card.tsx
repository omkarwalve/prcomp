//   ___                    _     _               
//  (  _`\                 ( )   ( )_             
//  | ( (_)   _ _  _ __   _| |   | ,_)  ___       
//  | |  _  /'_` )( '__)/'_` |   | |  /',__)(`\/')
//  | (_( )( (_| || |  ( (_| | _ | |_ \__, \ >  < 
//  (____/'`\__,_)(_)  `\__,_)(_)`\__)(____/(_/\_)
//                                                
//                                                

import React from 'react';

// Cog Imports
import Product, { ShortProduct } from 'Components/Listings/cogs/product';
import periodParser from './parser';
import { Checkbox, cmpActions } from '../Compare/Compare';
import { Add, cartActions } from 'Components/Assets/Cart/Cart';
import Store from 'Components/Assets/Stores/Stores';

import { ReactComponent as Specs } from './assets/specs.svg';
//import { ReactComponent as Rating } from './assets/rating.svg';
import { ReactComponent as Replace } from './assets/replace.svg';
import { ReactComponent as Warranty } from './assets/warranty.svg';

// CSS
import './card.css';

// -- Helper Functions
/** `Ignore a Parent's Click` */
const ignoreClick = (e: React.MouseEvent<any>) => { e.stopPropagation(); }

// -- Helper Components
const Name = ({name,layout}: {name: string,layout: string}) => {
  let maxchar: number;
    (layout == "compact") 
      ? maxchar = 59
      : (layout == "flow" || layout == "versus") 
        ? maxchar = 105 
        : (layout == "big") 
          ? maxchar = 208 
          : maxchar = 0
  
      if(name.length > maxchar) {
         const _name = `${name.slice(0,maxchar)}...`;
         return (
            <h4 className="product-name-sized">
               {_name}
               <span>{name}</span>
            </h4>
         )
      }
      else { return ( <h4 className="product-name">{name}</h4> )}
};

const Specifications = ({specs}: {specs: object | null}) => {
  return(
    <span className="product-specs-container">
      <Specs className="card-icon"/>
      <span className="product-specs-floaty" onClick={ignoreClick}>
        <h3>Specifications</h3>
        {(specs) 
            && (
                <div className="product-specs">
                  <table className="product-specs-table">
                    {
                      Object.keys(specs).map(key => {
                        return(
                          <tr>
                            <td>{key}</td> <td>{specs[key as keyof typeof specs]}</td>
                          </tr>
                        )
                      })
                    }
                  </table>
                </div>
               ) 
            || ( <h4 className="unavailable">Unavailable</h4> )
        }
      </span>
    </span>
  )
}

// # -- Product Card -- #
interface CardProps {
  product: Product;
  layout: string;
  setCmp: React.Dispatch<cmpActions>;
  setCartItems: React.Dispatch<cartActions>;
}
const Card = ({product,layout,setCmp,setCartItems}: CardProps) => {
  if (product.price.display === "Not Available" || product.cloaked) {
    return null;
  }
  else {
    //const stores: Map<string, JSX.Element> = new Map([
      //["Amazon"      , <Amazon      />],
      //["Flipkart"    , <Flipkart    />],
      //["Urbanladder" , <Urbanladder />],
    //]);

    const cardClick = (e: React.MouseEvent<HTMLDivElement>) => {
      e.preventDefault();
      alert("Details pane still in development :)");
    }
    const storeClick = (e: React.MouseEvent<HTMLSpanElement>) => {
      e.stopPropagation(); 
      window.open(product.url)
    }

        //<a href={product.url} target="_blank" rel="noreferrer">
        //</a>
    return (
      <div className={`product-card ${layout}`} onClick={cardClick}>

  {/*IMAGE*/}  <img className="product-image" src={product.img} alt="error" />
  {/*PNAME*/}  <Name name={product.name} layout={layout} />
  {/*RETPL*/}  <span className="product-retpolicy"><Replace  className="card-icon" />{(layout == "compact") ? periodParser(product.return_replace) : product.return_replace ?? '-'}</span>
  {/*WARPL*/}  <span className="product-warpolicy"><Warranty className="card-icon" />{(layout == "compact") ? periodParser(product.warranty) : product.warranty ?? '-' }            </span>
        {/*STORE*/}  <span className="product-store" onClick={storeClick}><Store.SVG store={product.store}/></span>
  {/*SPECS*/}  <Specifications specs={product.specs} />
  {/* CMP */}  <Checkbox pid={product.id} setCompare={setCmp}/>
  {/*CART */}  <Add product={product.shorten()} setCart={setCartItems} />
  {/*PRICE*/}  <span className="product-price">{product.price.display}</span>

      </div>
    )
  }
}


export default Card;
