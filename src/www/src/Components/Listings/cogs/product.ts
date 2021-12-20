
class Product {
    name: string;
    store: string;
    return_replace: string | null;
    warranty: string | null;
    specs: object | null;
    price: string;
    img: string;
    url: string;
    id: string;

    constructor(object: any) {
      this.name = object.name;
      this.store = object.store;
      this.return_replace = (object.warranty != "❔") ? object.return_replace : null;
      this.warranty = (object.warranty != "❔") ? object.warranty : null;
      this.specs = (object.specs != "❔") ? JSON.parse(object.specs) : null;
      this.price = object.price;
      this.img = object.img;
      this.url = object.url;
      this.id = object.id;
    }

    static from(object: any) : Product[] {
      return object?.listings.map((productdata: any)  => {
        return new Product(productdata);
      });
    }
    shorten(): ShortProduct {
      return new ShortProduct(this);
    }
}

/**
 * `ShortProduct` - A mini product dataclass for
 *  places where only basal product information is required
 * @property {string} name  - Product name
 * @property {string} image - Product image
 * @property {string} price - Product price
 * @property {string} store - Product store
 * @property {string} url   - Product url
 */
class ShortProduct {
    name: string;
    image: string;
    price: string;
    store: string;
    url: string;

    constructor(product: Product) {
        this.name  = product.name;
        this.image = product.img;
        this.price = product.price;
        this.store = product.store;
        this.url   = product.url;
    }
}

export { ShortProduct }
export default Product;
