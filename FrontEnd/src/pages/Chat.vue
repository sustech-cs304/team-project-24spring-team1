<template>
  <vue-advanced-chat
    :current-user-id="currentUserId"
    :rooms="JSON.stringify(rooms)"
    :rooms-order="JSON.stringify(roomsOrder)"
    :messages="JSON.stringify(messages)"
    :messages-loaded="JSON.stringify(messagesLoaded)"
    :rooms-loaded="JSON.stringify(roomsLoaded)"
    :show-add-room="JSON.stringify(showAddRoom)"
    :show-search="JSON.stringify(showSearch)"
    :room-actions="JSON.stringify(roomActions)"
    :message-actions="JSON.stringify(messageActions)"
    :show-new-messages-divider="JSON.stringify(showNewMessagesDivider)"
    :show-reaction-emojis="JSON.stringify(showReactionEmojis)"
    @fetch-messages="fetchMessages($event.detail[0])"
    @send-message="sendMessage($event.detail[0], $event.detail[1], $event.detail[2])"
  />
</template>

<script>
/**
     * AI-generated-content
     * tool: ChatGPT
     * version: 3.5
     * usage: I used the prompt "generate a chat page", and
     * directly copy the code from its response
     */
import axios from 'axios'
import { register } from 'vue-advanced-chat'
register()
export default {
  data() {
    return {
      currentRoom: null,
      currentUserId: localStorage.getItem('id'),
      token: localStorage.getItem('token'),
      chat: [],
      rooms: [],
      allRooms: [],
      roomsOrder: "desc",
      messages: [],
      messagesLoaded: true,
      roomsLoaded: true,
      roomActions: [],
      showAddRoom: false,
      showSearch: true,
      showNewMessagesDivider: false,
      showReactionEmojis: false,
      messageActions: [],
      intervalId: null
    }
  },
  mounted() {
    this.fetchChats()
  },
  beforeDestroy() {
    if (this.intervalId) {
      clearInterval(this.intervalId)
    }
  },
  methods: {
    imageURL(avatar) {
      return `https://backend.sustech.me/uploads/${avatar}.webp`
    },
    async fetchChats() {
      try {
        const response = await axios.get('https://backend.sustech.me/api/chat', {
          headers: {
            Authorization: `Bearer ${this.token}`
          },
          params: {
            page: 1
          }
        })

        this.chat = response.data.chats

        for (const chat of this.chat) {
          await this.fetchChatMembers(chat)
          await this.fetchChatMessages(chat)
        }

        this.constructRooms()
        this.allRooms = this.rooms

        // 启动定时器每秒检查新消息
        this.intervalId = setInterval(this.checkNewMessages, 2000)
      } catch (error) {
        console.error('Error fetching chats:', error)
      }
    },
    async fetchChatMembers(chat) {
      try {
        const response = await axios.get(`https://backend.sustech.me/api/chat/${chat.id}/member`, {
          headers: {
            Authorization: `Bearer ${this.token}`
          },
          params: {
            page: 1
          }
        })
        chat.members = response.data.members
      } catch (error) {
        console.error(`Error fetching members for chat ${chat.id}:`, error)
      }
    },
    async fetchChatMessages(chat) {
      try {
        const response = await axios.get(`https://backend.sustech.me/api/chat/${chat.id}/message`, {
          headers: {
            Authorization: `Bearer ${this.token}`
          },
          params: {
            page: 1
          }
        })
        chat.messages = response.data.messages
      } catch (error) {
        console.error(`Error fetching messages for chat ${chat.id}:`, error)
      }
    },
    async fetchUsername(accountId) {
      try {
        const response = await axios.get(`https://backend.sustech.me/api/account/${accountId}/profile`, {
          headers: {
            Authorization: `Bearer ${this.token}`
          }
        })
        return response.data.name
      } catch (error) {
        console.error(`Error fetching username for account ${accountId}:`, error)
        return ''
      }
    },
    async constructRooms() {
      const rooms = await Promise.all(this.chat.map(async chat => {
        const lastMessage = chat.messages.reduce((latest, message) => {
          return new Date(message.created_at) > new Date(latest.created_at) ? message : latest
        }, chat.messages[0])

        const users = chat.members.map(member => ({
          _id: member.id.toString(),
          username: member.name,
          avatar: this.imageURL(member.avatar),
          status: {
            state: 'online',
            lastChanged: '2024-05-20'
          }
        }))

        const otherUser = users.find(user => user._id !== this.currentUserId)
        const roomName = otherUser ? otherUser.username : `Room ${chat.id}`
        const roomAvatar = otherUser ? otherUser.avatar : ''
        const username = await this.fetchUsername(lastMessage.account_id)

        return {
          roomId: chat.id.toString(),
          roomName: roomName,
          avatar: roomAvatar,
          unreadCount: 0,
          index: new Date(lastMessage.created_at).getTime(),
          lastMessage: {
            _id: lastMessage.id.toString(),
            content: lastMessage.content.split('```')[0].trim(),
            senderId: lastMessage.account_id.toString(),
            username: username,
            timestamp: new Date(lastMessage.created_at).toLocaleTimeString(),
            saved: true,
            distributed: true,
            seen: false,
            new: true
          },
          users: users,
          typingUsers: [Math.min(...chat.members.map(member => member.id))]
        }
      }))

      this.rooms = rooms
    },
    async checkNewMessages() {
      try {
        const response = await axios.get('https://backend.sustech.me/api/chat', {
          headers: {
            Authorization: `Bearer ${this.token}`
          },
          params: {
            page: 1
          }
        })

        const newChats = response.data.chats
        let roomsChanged = false

        // 检查房间数量是否变化
        if (newChats.length !== this.chat.length) {
          roomsChanged = true
          this.chat = newChats

          // 重新获取所有房间的成员和消息
          for (const chat of this.chat) {
            await this.fetchChatMembers(chat)
            await this.fetchChatMessages(chat)
          }

          await this.constructRooms()
        } else {
          // 检查每个房间的消息数量是否变化
          for (let i = 0; i < newChats.length; i++) {
            const chatId = newChats[i].id
            const oldMessages = this.chat[i].messages
            console.log('oldMessage =', oldMessages)

            const messageResponse = await axios.get(`https://backend.sustech.me/api/chat/${chatId}/message`, {
              headers: {
                Authorization: `Bearer ${this.token}`
              },
              params: {
                page: 1
              }
            })

            const newMessages = messageResponse.data.messages
            console.log('newMessages =', newMessages)

            if (newMessages.length !== oldMessages.length) {
              roomsChanged = true
              console.log('chatId =', chatId)
              console.log('currentRoom =', this.currentRoom)
              console.log('currentRoomId =', this.currentRoom.roomId)
              if (chatId == this.currentRoom.roomId) {
                await this.fetchMessages({ room: this.currentRoom })
              }
              this.chat[i].messages = newMessages

              // 更新对应房间的 lastMessage 字段
              const lastMessage = newMessages[0]
              const username = await this.fetchUsername(lastMessage.account_id)
              console.log('fetch username =', username)
              const roomIndex = this.rooms.findIndex(r => r.roomId === newChats[i].id.toString())
              if (roomIndex !== -1) {
                this.rooms[roomIndex].lastMessage = {
                  _id: lastMessage.id.toString(),
                  content: lastMessage.content.split('```')[0].trim(),
                  senderId: lastMessage.account_id.toString(),
                  username: username,
                  timestamp: new Date(lastMessage.created_at).toLocaleTimeString(),
                  saved: true,
                  distributed: true,
                  seen: false,
                  new: true
                }
                this.rooms[roomIndex].index = new Date(lastMessage.created_at).getTime()
                console.log('change laseMessage to', this.rooms[roomIndex].lastMessage)
              }
              // console.log('lastMessage =' ,lastMessage)
            }
          }
        }
        if (roomsChanged) {
          await this.constructRooms()
        }
      } catch (error) {
        console.error('Error checking new messages:', error)
      }
    },
    async fetchMessages({ room, options = {} }) {
      console.log('fetchMessages room =', room)
      this.currentRoom = room
      this.messagesLoaded = false
      console.log(`Fetching messages for room:`, room.roomId)
      console.log('this.currentRoom =', this.currentRoom)

      try {
        const response = await axios.get(`https://backend.sustech.me/api/chat/${room.roomId}/message`, {
          headers: {
            Authorization: `Bearer ${this.token}`
          },
          params: {
            page: options.page || 1
          }
        })

        const chatMessages = await Promise.all(response.data.messages.map(async message => {
          const sender = this.chat.find(chat => chat.id.toString() === room.roomId).members.find(member => member.id === message.account_id)
          const username = await this.fetchUsername(message.account_id)
          const files = []

          console.log('fetch message content =', message.content)
          const contentParts = message.content.split('```')
          const content = contentParts[0].trim()
          
          if (contentParts.length > 1) {
            try {
              const fileContent = contentParts[1].trim()
              const fileArray = JSON.parse(fileContent)
              console.log('fileArray =', fileArray)

              fileArray.forEach(file => {
                if (file.type.startsWith('image/')) {
                  console.log('fetch message with image file =', file)
                  axios.get(file.url, { responseType: 'blob' })
                    .then(response => {
                      const reader = new FileReader()
                      reader.onloadend = () => {
                        file.preview = reader.result
                        files.push(file)
                      }
                      reader.readAsDataURL(response.data)
                    })
                    .catch(error => {
                      console.error('Error fetching image file:', error)
                    })
                } else {
                  console.log('fetch message with non-image file =', file)
                  files.push(file)
                }
              })
            } catch (error) {
              console.error('Error parsing file content:', error)
            }
          }

          return {
            _id: message.id.toString(),
            content: content,
            index: 1000 - message.id,
            senderId: message.account_id.toString(),
            username: username,
            avatar: sender ? this.imageURL(sender.avatar) : '',
            date: new Date(message.created_at).toLocaleDateString(),
            timestamp: new Date(message.created_at).toLocaleTimeString(),
            system: false,
            saved: true,
            distributed: true,
            seen: false,
            deleted: false,
            disableActions: false,
            disableReactions: false,
            files: files,
            reactions: {},
            replyMessage: null
          }
        }))

        this.messages = chatMessages

        // Update the corresponding room's lastMessage and index fields
        const lastMessage = chatMessages[0]
        const roomIndex = this.rooms.findIndex(r => r.roomId === room.roomId)
        if (roomIndex !== -1) {
          this.rooms[roomIndex].lastMessage = {
            _id: lastMessage._id,
            content: lastMessage.content.split('```')[0].trim(),
            senderId: lastMessage.senderId,
            username: lastMessage.username,
            timestamp: lastMessage.timestamp,
            saved: lastMessage.saved,
            distributed: lastMessage.distributed,
            seen: lastMessage.seen,
            new: lastMessage.new
          }
          this.rooms[roomIndex].index = new Date(lastMessage.timestamp).getTime()
        }

        this.messagesLoaded = true
      } catch (error) {
        console.error(`Error fetching messages for room ${room.roomId}:`, error)
      }
    },
    async sendMessage({ roomId, content, files = [], replyMessage = {}, usersTag = {} }) {
      console.log('sendMessage: roomID = ', roomId, 'content = ', content)

      const uploadedFiles = []
      console.log('file is', files)

      if (files != null) {
        const fileUploadPromises = files.map(file => new Promise((resolve, reject) => {
          const formData = new FormData()
          formData.append('file', file.blob)

          axios.post('https://backend.sustech.me/upload', formData, {
            headers: {
              'Content-Type': 'multipart/form-data',
              Authorization: `Bearer ${this.token}`
            }
          })
          .then(response => {
            const fileUrl = 'https://backend.sustech.me' + response.data
            console.log('upload files response', response.data)

            uploadedFiles.push({
              name: file.name,
              size: file.size,
              type: file.type,
              url: fileUrl,
              extension: file.extension,
              progress: 100,
              preview: '' // 不包含 preview 字段
            })
            resolve()
          })
          .catch(error => {
            console.error('Error uploading file:', error)
            reject(error)
          })
        }))

        try {
          await Promise.all(fileUploadPromises)
          const filesContent = '```' + JSON.stringify(uploadedFiles) + '```'
          content += '\n' + filesContent
          console.log('upload file content =', content)
        } catch (error) {
          console.error('Error in file upload promises:', error)
        }
      }

      try {
        const response = await axios.post(`https://backend.sustech.me/api/chat/${roomId}/message`, {
          content: content
        }, {
          headers: {
            Authorization: `Bearer ${this.token}`
          }
        })

        console.log('Response from sendMessage:', response.data)
        const room = this.rooms.find(r => r.roomId === roomId.toString())
        if (room) {
          this.fetchMessages({ room: room })
        } else {
          console.error(`Room with id ${roomId} not found`)
        }
      } catch (error) {
        console.error(`Error sending message to room ${roomId}:`, error)
      }
    }
  }
}
</script>
