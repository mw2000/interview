import requests
import sys


def get_author_data(authorUrl):
    request_url = "https://openlibrary.org/" + authorUrl + ".json"
    response = requests.get(request_url)

    # If personal name doesn't exist in the object
    # then return some other param

    return response.json().get('name')

def get_data(isbn):
    data_dictionary = {}
    
    request_url = "https://openlibrary.org/isbn/" + isbn + ".json"
    response = requests.get(request_url)

    data_dictionary['metadata'] = {"title": response.json().get('title')}

    for author in response.json().get('authors'):
        if "authors" in data_dictionary:
            data_dictionary['authors'].append(get_author_data(author.get('key')))
        else:
            data_dictionary['authors'] = [get_author_data(author.get('key'))]

    data_dictionary['publish_date'] = response.json().get('publish_date')
    return data_dictionary

print(get_data(sys.argv[1]))
