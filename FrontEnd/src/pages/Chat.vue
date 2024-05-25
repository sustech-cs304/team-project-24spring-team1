<template>
  <!-- <div>
    <div>
      <h3>Rooms</h3>
      <pre>{{ JSON.stringify(rooms, null, 2) }}</pre>
    </div>
    <div>
      <h3>Messages</h3>
      <pre>{{ JSON.stringify(messages, null, 2) }}</pre>
    </div>
    <div>
      <h3>Chat</h3>
      <pre>{{ JSON.stringify(chat, null, 2) }}</pre>
    </div>
  </div> -->
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
    :show-new-messages-driver=false
    @fetch-messages="fetchMessages($event.detail[0])"
    @send-message="sendMessage($event.detail[0], $event.detail[1])"
  />
</template>

<script>
import axios from 'axios'
import { register } from 'vue-advanced-chat'
register()

export default {
  data() {
    return {
      currentUserId: localStorage.getItem('id'),
      token: localStorage.getItem('token'),
      chat: [],
      rooms: [],
      allRooms: [],
      roomsOrder: "desc",
      messages: [],
      messagesLoaded: true,
      roomsLoaded: true,
      roomActions: [
        // { name: 'inviteUser', title: 'Invite User' },
        // { name: 'removeUser', title: 'Remove User' },
        // { name: 'deleteRoom', title: 'Delete Room' }
      ],
      showAddRoom: false,
      showSearch: true,
      messageActions: [
        // {
        //   name: 'replyMessage',
        //   title: 'Reply'
        // },
        // {
        //   name: 'editMessage',
        //   title: 'Edit Message',
        //   onlyMe: true
        // },
      ],
    }
  },
  mounted() {
    this.fetchChats()
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
        this.fetchMessages({ room: this.rooms[0] }) // 初始化时加载第一个房间的消息
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
    constructRooms() {
      const rooms = this.chat.map(chat => {
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

        // 找到第一个不是当前用户的用户，并使用该用户的用户名作为房间名
        const otherUser = users.find(user => user._id !== this.currentUserId)
        const roomName = otherUser ? otherUser.username : `Room ${chat.id}`

        const roomAvatar = otherUser ? otherUser.avatar : ''

        return {
          roomId: chat.id.toString(),
          roomName: roomName,
          avatar: roomAvatar,
          unreadCount: 0,
          index: new Date(lastMessage.created_at).getTime(),
          lastMessage: {
            _id: lastMessage.id.toString(),
            content: lastMessage.content,
            senderId: lastMessage.account_id.toString(),
            username: chat.members.find(member => member.id === lastMessage.account_id)?.name || '',
            timestamp: new Date(lastMessage.created_at).toLocaleTimeString(),
            saved: true,
            distributed: true,
            seen: false,
            new: true
          },
          users: users,
          typingUsers: [Math.min(...chat.members.map(member => member.id))]
        }
      })

      // 使用数组赋值而不是push方法
      this.rooms = rooms
    },
    async fetchMessages({ room, options = {} }) {
      this.messagesLoaded = false
      console.log(`Fetching messages for room:`, room)

      try {
        const response = await axios.get(`https://backend.sustech.me/api/chat/${room.roomId}/message`, {
          headers: {
            Authorization: `Bearer ${this.token}`
          },
          params: {
            page: options.page || 1 // 使用传入的分页参数
          }
        })

        const chatMessages = response.data.messages.map(message => {
          const sender = this.chat.find(chat => chat.id.toString() === room.roomId).members.find(member => member.id === message.account_id)
          return {
            _id: message.id.toString(),
            content: message.content,
            index: 1000 - message.id,
            senderId: message.account_id.toString(),
            username: sender ? sender.name : '',
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
            files: [],
            reactions: {},
            replyMessage: null
          }
        })

        // 使用数组赋值而不是push方法
        this.messages = chatMessages
        this.messagesLoaded = true
      } catch (error) {
        console.error(`Error fetching messages for room ${room.roomId}:`, error)
      }
    },
    async sendMessage({ roomId, content, files = {}, replyMessage = {}, usersTag = {} }) {
      console.log('sendMessage: roomID = ', roomId, 'content = ', content)
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
    },
  }
}
</script>
