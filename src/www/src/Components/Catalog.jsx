import React from 'react'

import './Catalog.css'
// All catalog logos
import { ReactComponent as Electronics } from './catalog/elx.svg';
import { ReactComponent as Furniture } from './catalog/fur.svg';
import { ReactComponent as Fashion } from './catalog/fas.svg';
import { ReactComponent as Grocery } from './catalog/gro.svg';
import { ReactComponent as Stationary } from './catalog/sta.svg';
import { ReactComponent as Garden } from './catalog/gar.svg';
import { ReactComponent as Toys } from './catalog/toy.svg';
import { ReactComponent as Books } from './catalog/bok.svg';

export default class Section extends React.Component {

                //<div className="containerRow1">
                //</div>
                //<div className="containerRow2">
                //</div>
                        //<img src="/catalog/elx.svg" alt="error" />
    render() {
        return (
            <div className="content">
                    <div className="category-block">
                        <Electronics />
                        <div className="catName">Electronics</div>
                    </div>    
                    
                    <div className="category-block">
                        <Furniture />
                        <div className="catName">Furniture</div>                    
                    </div>

                    <div className="category-block">
                        <Fashion />
                        <div className="catName">Fashion</div>
                    </div>

                    <div className="category-block">
                        <Grocery />
                        <div className="catName">Grocery</div>
                    </div>

                    <div className="category-block">
                        <Stationary />
                        <div className="catName">Stationary</div>
                    </div>
                    
                    <div className="category-block">  
                        <Garden />
                        <div className="catName">Garden</div>
                    </div>
                    
                    <div className="category-block">                    
                        <Toys />
                        <div className="catName">Toys</div>
                    </div>
                    
                    <div className="category-block">
                        <Books />
                        <div className="catName">Books</div>
                    </div>
            </div>
        )
    }
}
