import React from 'react';

import './Productdef.css';
import { ReactComponent as Rating } from './list/rating.svg';
import { ReactComponent as Replace } from './list/replace.svg';
import { ReactComponent as Warranty } from './list/warranty.svg';

const Name = ({name}) => {
      if(name.length > 105) {
         const pname = `${name.slice(0,100)}...`;
         return (
            <div className="tooltipName">
               <h4 className="product-name">{pname}</h4>
               <span className="tooltipN_whole">{name}</span>
            </div>
         )
      }
      else { return ( <h4 className="product-name">{name}</h4> )}
};

const Productdef = ({ name, price, store, rating, returnPolicy, warranty, url}) => {


    return (
           <div className="product-info-grid">
               <Name name={name} />
               <p className="pricetag">{price}</p>
               <img className="storeIcon" src={store} alt="error"></img>
               <p className="rating">
                  <Rating className="servIcon"/>
                  {rating}</p>
               <p className="returnpolicy">
                  <Replace className="servIcon"/>
                  {returnPolicy}</p>
               <p className="warranty">
                  <Warranty className="servIcon"/>
                  {warranty}</p>
           </div>
    )
}

export default Productdef;
