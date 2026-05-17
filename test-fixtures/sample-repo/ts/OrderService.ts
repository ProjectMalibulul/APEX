import { PaymentService } from "./PaymentService";

export class OrderService {
  constructor(private readonly payments: PaymentService) {}

  placeOrder(): string {
    return this.payments.capture();
  }
}

