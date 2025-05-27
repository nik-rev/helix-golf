# snake_case to camelCase

Rename all fields to be camelCase.

## Before

```js
const user_profile = {
  first_name: "John",
  last_name: "Doe",
  birth_date: "1990-05-15",
  email_address: "john_doe@example.com",
  phone_number: "555-123-4567",
  mailing_address: {
    street_name: "Main Street",
    house_number: 123,
    apartment_unit: "4B",
    zip_code: "10001",
    city_name: "New York",
  },
};
```

## After

```js
const userProfile = {
  firstName: "John",
  lastName: "Doe",
  birthDate: "1990-05-15",
  emailAddress: "john_doe@example.com",
  phoneNumber: "555-123-4567",
  mailingAddress: {
    streetName: "Main Street",
    houseNumber: 123,
    apartmentUnit: "4B",
    zipCode: "10001",
    cityName: "New York",
  },
};
```

## Command

```
%s_<enter>5)<alt-,>d~
```

1. `%` selects the entire file
2. `s_<enter>` selects all underscores
3. `5)` rotates the main selection forward 5 times
4. `<alt-,>` removes the primary selection - the lone underscore we want to keep
5. `d` deletes the selections
6. `~` toggles the case
