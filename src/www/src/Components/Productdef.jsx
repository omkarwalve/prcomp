import React from 'react';

import './Productdef.css';
import { ReactComponent as Rating } from './list/rating.svg';
import { ReactComponent as Replace } from './list/replace.svg';
import { ReactComponent as Warranty } from './list/warranty.svg';

const Productdef = ({ name, price, store, rating, returnPolicy, warranty, url}) => {
    return (
        <div className="header_container">
         <a href={url} target="_blank" rel="noreferrer">
            <h4 className="name">{name}</h4>
            <p className="pricetag">{price}</p>
            <p><img className="storeIcon" src={store} alt="error"></img></p>
            <p className="rating">
                 <Rating className="servIcon"/>
               {rating}</p>
            <p className="returnpolicy">
                 <Replace className="servIcon"/>
               {returnPolicy}</p>
            <p className="warranty">
                 <Warranty className="servIcon"/>
            {warranty}</p>
         </a>
        </div>
    )
}

export default Productdef;
