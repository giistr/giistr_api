API documentation
=================

## Authentication

There is no account creation logic for this api, you have to embedded in all your request a custom header specifying your github user token.
Here is the header name: "X-Github-Token", and should contain a string.

The server will store your github token to authenticate your user.
If the token you specify is not stored in database, we will ask github if this token is associated to some user, and then create a giistr user, with your github token, and the github user_id (we can get it from your token).
Again if your token is not stored, we ask github to know to whom it is associated. In the case github return to our server a user_id associated to this token that we already know, we just update the actual old token, assuming the old one is no longer available.

If the token already exist, we just assume you are the right user.

Here is a list of the different routes served by the giistr backend server.

# Users

## Get a specific User
This is mainly done for debug purpose.

This route contains an url parameter (id) which is a giistr user_id.
Return a specific user in database.

### Route
GET */api/v1/user/:id*

### Request
None

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "token_id": string,
  "github_user_id": string,
}
```
Where:
* `id`, is an internal giistr id generated from our own
* `token_id`, is a stored github token
* `github_user_id`, is the github id of your user

## List all users
See previous route, the behavior is the same, but return the whole list of users.

### Route
GET */api/v1/users*

### Request
None

### Response
A list of the same object than the route for an user.

# Tags

## Get a specific Tag
This route allow you to get a specific tag from it id

### Route
Get */api/v1/tag/:id*

### Request
None

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "name": string,
}
```
Where:
* `user_id`, is the id of the user, the tag is associated to.
* `name`, is the actual name of the tag

## Get the whole tag list of your user
This route allow you to get all the tag associated to your current user.
We will use the github token you specify in you request, to get your giistr internal user
then list all you tagB

### Route
GET */api/v1/tags*

### Request
None

### Response
The same object than to get a single user, but as a list


## Create a tag
This route will create a new tag for the user you make the request with.

### Route
POST */api/v1/tag*

### Request
```json
{
  "name": string,
}
```

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "name": string,
}
```
The giistr object representation the created tag is returned.

### Update a tag
This route will update an existing tag.
You cannot edit a tag associated to a different user than the one link to the github token associated to your request

### Route
PUT */api/v1/tag*

### Request
```json
{
  "id": string
  "name": optional string,
}
```

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "name": string,
}
```
The giistr object representation of the updated tag

## Delete a tag
This route allow you to remove a tag.
You cannot delete a tag associated to a different user than the one link to the github token you use the this request

### Route
DELETE */api/v1/tag/:id*

### Request
None

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "name": string,
}
```
The deleted object.

# Repositories

## Get a specific Repo
This route allow you to get a specific repo from it id, you cannot get a associated to another user.

### Route
GET */api/v1/repo/:id*

### Request
None

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "github_repo_id": string,
}
```
Where:
* `user_id`, is the id of the user, the tag is associated to.
* `github_repo_id`, is the github internal repo identifier

## Get all the repositories
This route return all the repositories associated to the user link to the github token for this request.

### Route
GET */api/v1/repos*

### Request
None

### Response
A list of the same type than the previous route.

## Create
Create a new repository for the user linked to the request.

### Route
POST */api/v1/repo*

### Request
```json
{
  "github_repo_id": string,
}
```

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "github_repo_id": string,
}
```
the created repo.

## Update
Update a repository for the user linked to the request, the repo owner must match the user associated the token in the request.

### Route
PUT */api/v1/repo*

### Request
```json
{
  "id": string
  "github_repo_id": optional string,
}
```

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "github_repo_id": string,
}
```
The updated repo

## Delete a Repo
This route allow you to remove a repo.
You cannot delete a repo associated to a different user than the one link to the github token you use the this request

### Route
DELETE */api/v1/repo/:id*

### Request
None

### Response
```json
{
  "id": string,
  "created_at": integer,
  "updated_at": integer,
  "user_id": string,
  "github_repo_id": string,
}
```
The deleted object.

# Associate a tag a repository

## Create
This will allow you to create a association between a tag and a repository. The github token of the request must link to the same user than the tag and repository.

### Route
POST */api/v1/tag/:tag_id/repo/:repo_id*
this route take to url parameters which are the id of the tag, and the id of repository

### Request
None

### Response
```json
{
  "assoc": {
    "id": string,
    "repo_id": string,
    "tag_id": string,
  },
  "repo": {
    "id": string,
    "created_at": integer,
    "updated_at": integer,
    "user_id": string,
    "github_repo_id": string,
  },
  "tag": {
    "id": string,
    "created_at": integer,
    "updated_at": integer,
    "user_id": string,
    "name": string,
  }
}
```

## Delete an association
This route allow you to delete an association of a tag with a repository

### Route
DELETE */api/v1/repo-tag-assoc/:id*
This route takes on url parameters which is the id of the association

### Request
None

### Response
```json
{
  "assoc": {
    "id": string,
    "repo_id": string,
    "tag_id": string,
  },
  "repo": {
    "id": string,
    "created_at": integer,
    "updated_at": integer,
    "user_id": string,
    "github_repo_id": string,
  },
  "tag": {
    "id": string,
    "created_at": integer,
    "updated_at": integer,
    "user_id": string,
    "name": string,
  }
}
```
The delete association

## List all tags for a repository
This will list all the tag for the specified repository.
The repository must be owned by the user linked to the github token of the request.

### Route
GET */api/v1/tags/repo/:id*

### Request
None

### Response
```json
[
  {
    "id": string,
    "created_at": integer,
    "updated_at": integer,
    "user_id": string,
    "name": string,
  },
  {...}
]
```
The list of all the tags associated to this repository

## List all repositories for a tag
This will list all the repositories which are associated to this tag.
The tag must be owned by the user linked to the github token of the request.

### Route
GET */api/v1/repos/tag/:id*

### Request
None

### Response
```json
[
  {
    "id": string,
    "created_at": integer,
    "updated_at": integer,
    "user_id": string,
    "github_repo_id": string,
  },
  {...}
]
```
The list of all the repositories associated to this repository

