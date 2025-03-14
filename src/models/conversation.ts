// import type { Conversation, Student } from '@prisma/client'

import type { Student } from "./student";

// export interface ExtendedConversation extends Conversation {
//   student?: Student
//   lastMessage?: string
// }

export interface Conversation {
  id: string;
  createdAt: string;
  updatedAt: string;
  title: string | null;
  studentName: string;
}

export interface ConversationWithStudent {
  id: string;
  createdAt: string;
  updatedAt: string;
  title: string | null;
  student_name: string;
  student?: Student;
  lastMessage?: string
}