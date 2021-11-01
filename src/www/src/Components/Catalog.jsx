import React from 'react'
import './Catalog.css'

export default class Section extends React.Component {

                //<div className="containerRow1">
                //</div>
                //<div className="containerRow2">
                //</div>
    render() {
        return (
            <div className="content">
                    <div className="category-block">
                        <img src="/electronic-category.jpg" alt="error" />
                        <div className="catName">Electronics</div>
                    </div>    
                    
                    <div className="category-block">
                        <img src="/furniture-category.jpg" alt="error" />
                        <div className="catName">Furniture</div>                    
                    </div>

                    <div className="category-block">
                        <img src="/fashion-category.jpg" alt="error" />
                        <div className="catName">Fashion</div>
                    </div>

                    <div className="category-block">
                        <img src="/grocery-category.jfif" alt="error" />
                        <div className="catName">Grocery</div>
                    </div>

                    <div className="category-block">
                        <img src="/stationary-category.jpg" alt="error" />
                        <div className="catName">Stationary</div>
                    </div>
                    
                    <div className="category-block">  
                        <img src="/garden-category.jpg" alt="error" />
                        <div className="catName">Garden</div>
                    </div>
                    
                    <div className="category-block">                    
                        <img src="/toys-category.jpg" alt="error" />
                        <div className="catName">Toys</div>
                    </div>
                    
                    <div className="category-block">
                        <img src="/books-category.jpg" alt="error" />
                        <div className="catName">Books</div>
                    </div>
            </div>
        )
    }
}
