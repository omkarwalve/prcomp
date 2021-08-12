# Nabu's Algorithm

1. Take `QUERY` and `CATEGORY` as input.
2. Loop through the `CATEGORY` file for the list of sites to be scraped. 
> File should look like 
> ```
> Amazon
> Flipkart
> Alibaba
> ```
3. Fetch the object file(`PROFILE`) for a website and...
    1. Level 1 Scraping
        - Make HTTP/API request to the query page with the `QUERY`.
        - Collect all listing urls into `LIST`.
    2. Level 2 Scraping
        - Make HTTP request to element of `LIST`.
            - Parse DOM tree and scrape out data from elements pointed by `PROFILE`, append in `JSON` format.
        - Proceed to next element in `LIST`.
4. Return the complete `JSON` as output.
