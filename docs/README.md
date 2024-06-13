# NoDrive
NoDrive is a cloud-based file storage and sharing platform that provides users with a secure and convenient way to store, access, and collaborate on their digital content.

## Use Cases
### Online Storage 
Users can upload and store a variety of file types, including documents, spreadsheets, presentations, images, and more, in the cloud.
### File Synchronization
 Files stored in No-Drive are automatically synchronized across devices, allowing users to access their files from laptops, tablets, smartphones, and other devices.
### Collaboration
 Multiple users can simultaneously view, edit, and comment on files stored in No-Drive, enabling real-time collaboration.
### File Sharing
 Users can easily share individual files or entire folders with others, with the ability to control permissions and access levels.
### Mobile Access
  No-Drive offers mobile apps for both iOS and Android, allowing users to access their files on the go.

## Brief rundown of implementation
No-Drive is built entirely with rust so as to maximise performance, speed, memory usage and security.

### Clients
No-Drive is built to be compatible with and available for all clients.
![Clients](./use-case/client.png "All client (Phone, Tablet, PC, etc.)")

### DB
No-Drive is implemented using a ```PostgreSQL database``` to manage user's data, the app's metadata, and other relevant information.
![Database](./use-case/db.png "PostgreSQL Database")

### File service
The file service, which governs how No-Drive manages files, is implemented using several components: ```shaku``` for Dependency injection and ```Tokio``` for the asynchronous runtime to name a few.
![File service](./use-case/file-service.png "File service")

### Notification service
The notification service, which manages notifications sent by and to No-Drive, is managed with ```rumqttc``` as ```MQTT client/server``` among other components.
![Notification service](./use-case/notification-service.png "Notification service")

### User service
The user service, which manages how No-Drive handles user information, is managed with ```OAuth2 server``` library called ```oxide-auth``` and other technologies.
![User service](./use-case/user-service.png "User service")

________________________________________________________________
For more on the technologies and approaches used for developping the No-Drive project, see the [technical documentation](./DOC.md)

## Contributing
We welcome contributions from the community to help improve No-Drive. If you're interested in contributing, please visit the [No-Drive GitHub repository](https://github.com/stephane-segning/no-drive/), fork the repository and submit a pull request. We'll be sure to look at it.

## License
No-Drive is released under the MIT License.
________________________________________________________________
For more on the technologies and approaches used for developping the No-Drive project, see the [technical documentation](./DOC.md)